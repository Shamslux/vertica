import { useEffect, useRef, useState } from "react";

import { useTranslation } from "../../i18n/useTranslation";
import { getSettings, updateSettings } from "./settingsClient";
import type {
  ApplicationSettings,
  LanguagePreference,
  ThemePreference,
} from "./settingsTypes";

type LoadState =
  | { status: "loading" }
  | { status: "ready"; settings: ApplicationSettings }
  | { status: "error"; message: string };

type SaveState = "idle" | "saving" | "saved" | "error";

function normalizeError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  return "An unexpected error occurred while processing settings.";
}

function isThemePreference(
  value: string,
): value is ThemePreference {
  return (
    value === "system" ||
    value === "light" ||
    value === "dark"
  );
}

function isLanguagePreference(
  value: string,
): value is LanguagePreference {
  return value === "en" || value === "pt-BR";
}

function applyTheme(theme: ThemePreference): void {
  document.documentElement.dataset.theme = theme;
  document.documentElement.style.colorScheme =
    theme === "system" ? "light dark" : theme;
}

export function SettingsPanel() {
  const {
    t,
    setLanguage: setInterfaceLanguage,
  } = useTranslation();

  const [loadState, setLoadState] = useState<LoadState>({
    status: "loading",
  });

  const [theme, setTheme] =
    useState<ThemePreference>("system");

  const [language, setLanguage] =
    useState<LanguagePreference>("en");

  const [saveState, setSaveState] =
    useState<SaveState>("idle");

  const [saveError, setSaveError] =
    useState<string | null>(null);

  const isMountedRef = useRef(true);
  const saveRequestIdRef = useRef(0);

  useEffect(() => {
    isMountedRef.current = true;

    async function load(): Promise<void> {
      try {
        const settings = await getSettings();

        if (!isMountedRef.current) {
          return;
        }

        setTheme(settings.theme);
        setLanguage(settings.language);
        setInterfaceLanguage(settings.language);

        applyTheme(settings.theme);

        setLoadState({
          status: "ready",
          settings,
        });

        setSaveState("saved");
      } catch (error) {
        if (!isMountedRef.current) {
          return;
        }

        setLoadState({
          status: "error",
          message: normalizeError(error),
        });

        setSaveState("error");
      }
    }

    void load();

    return () => {
      isMountedRef.current = false;
    };
  }, [setInterfaceLanguage]);

  async function persistSettings(
    nextTheme: ThemePreference,
    nextLanguage: LanguagePreference,
    previousTheme: ThemePreference,
    previousLanguage: LanguagePreference,
  ): Promise<void> {
    const requestId = saveRequestIdRef.current + 1;
    saveRequestIdRef.current = requestId;

    setSaveState("saving");
    setSaveError(null);

    try {
      const settings = await updateSettings({
        theme: nextTheme,
        language: nextLanguage,
      });

      if (
        !isMountedRef.current ||
        requestId !== saveRequestIdRef.current
      ) {
        return;
      }

      setTheme(settings.theme);
      setLanguage(settings.language);
      setInterfaceLanguage(settings.language);

      applyTheme(settings.theme);

      setLoadState({
        status: "ready",
        settings,
      });

      setSaveState("saved");
    } catch (error) {
      if (
        !isMountedRef.current ||
        requestId !== saveRequestIdRef.current
      ) {
        return;
      }

      setTheme(previousTheme);
      setLanguage(previousLanguage);
      setInterfaceLanguage(previousLanguage);

      applyTheme(previousTheme);

      setSaveError(normalizeError(error));
      setSaveState("error");
    }
  }

  function handleThemeChange(value: string): void {
    if (!isThemePreference(value) || value === theme) {
      return;
    }

    const previousTheme = theme;
    const previousLanguage = language;

    setTheme(value);
    applyTheme(value);

    void persistSettings(
      value,
      language,
      previousTheme,
      previousLanguage,
    );
  }

  function handleLanguageChange(value: string): void {
    if (
      !isLanguagePreference(value) ||
      value === language
    ) {
      return;
    }

    const previousTheme = theme;
    const previousLanguage = language;

    setLanguage(value);
    setInterfaceLanguage(value);

    void persistSettings(
      theme,
      value,
      previousTheme,
      previousLanguage,
    );
  }

  function getStatusLabel(): string {
    if (loadState.status === "loading") {
      return t("settings.loading");
    }

    if (saveState === "saving") {
      return t("settings.saving");
    }

    if (
      saveState === "error" ||
      loadState.status === "error"
    ) {
      return t("settings.failed");
    }

    return t("settings.saved");
  }

  const isLoading = loadState.status === "loading";
  const isSaving = saveState === "saving";
  const controlsDisabled = isLoading || isSaving;

  return (
    <section
      className="status-panel settings-panel"
      aria-labelledby="application-settings-title"
      aria-live="polite"
      aria-busy={isLoading || isSaving}
    >
      <div className="panel-heading">
        <div>
          <p className="section-label">
            {t("settings.preferences")}
          </p>

          <h2 id="application-settings-title">
            {t("settings.applicationSettings")}
          </h2>
        </div>

        <span
          className={`status-badge ${
            saveState === "error" ||
            loadState.status === "error"
              ? "status-badge--error"
              : ""
          }`}
        >
          {getStatusLabel()}
        </span>
      </div>

      {isLoading && (
        <p className="state-message">
          {t("settings.loadingSettings")}
        </p>
      )}

      {loadState.status === "error" && (
        <div className="error-message" role="alert">
          <strong>{t("settings.loadError")}</strong>
          <span>{loadState.message}</span>
        </div>
      )}

      {saveError && (
        <div className="error-message" role="alert">
          <strong>{t("settings.saveError")}</strong>
          <span>{saveError}</span>
        </div>
      )}

      {loadState.status === "ready" && (
        <div className="settings-form">
          <div className="settings-group">
            <div className="settings-copy">
              <label htmlFor="theme-preference">
                {t("settings.theme")}
              </label>

              <p>{t("settings.themeDescription")}</p>
            </div>

            <select
              id="theme-preference"
              value={theme}
              onChange={(event) =>
                handleThemeChange(event.target.value)
              }
              disabled={controlsDisabled}
            >
              <option value="system">
                {t("settings.system")}
              </option>

              <option value="light">
                {t("settings.light")}
              </option>

              <option value="dark">
                {t("settings.dark")}
              </option>
            </select>
          </div>

          <div className="settings-group">
            <div className="settings-copy">
              <label htmlFor="language-preference">
                {t("settings.language")}
              </label>

              <p>{t("settings.languageDescription")}</p>
            </div>

            <select
              id="language-preference"
              value={language}
              onChange={(event) =>
                handleLanguageChange(event.target.value)
              }
              disabled={controlsDisabled}
            >
              <option value="en">
                {t("settings.english")}
              </option>

              <option value="pt-BR">
                {t("settings.portugueseBrazil")}
              </option>
            </select>
          </div>

          <div className="settings-storage">
            <div>
              <span>{t("settings.storage")}</span>

              <strong>
                {t("settings.activeProfileDatabase")}
              </strong>
            </div>

            <span className="settings-storage-status">
              {t("settings.persistent")}
            </span>
          </div>

          <p className="settings-hint">
            {t("settings.changesSavedAutomatically")}
          </p>
        </div>
      )}
    </section>
  );
}

