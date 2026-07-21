import { useEffect, useState } from "react";

import "./App.css";
import { getApplicationInfo } from "./features/application-info/applicationInfoClient";
import type { ApplicationInfo } from "./features/application-info/applicationInfoTypes";
import { getDatabaseDiagnostics } from "./features/database-diagnostics/databaseDiagnosticsClient";
import type { DatabaseDiagnostics } from "./features/database-diagnostics/databaseDiagnosticsTypes";
import { SettingsPanel } from "./features/settings/SettingsPanel";
import { useTranslation } from "./i18n/useTranslation";

type LoadState<T> =
  | { status: "loading" }
  | { status: "success"; data: T }
  | { status: "error"; message: string };

function normalizeError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  return "An unexpected error occurred.";
}

function formatValue(value: string): string {
  return value.replaceAll("_", " ");
}

function App() {
  const { t } = useTranslation();

  const [applicationInfoState, setApplicationInfoState] = useState<
    LoadState<ApplicationInfo>
  >({
    status: "loading",
  });

  const [databaseDiagnosticsState, setDatabaseDiagnosticsState] = useState<
    LoadState<DatabaseDiagnostics>
  >({
    status: "loading",
  });

  useEffect(() => {
    let isMounted = true;

    async function loadApplicationInfo(): Promise<void> {
      try {
        const applicationInfo = await getApplicationInfo();

        if (!isMounted) {
          return;
        }

        setApplicationInfoState({
          status: "success",
          data: applicationInfo,
        });
      } catch (error) {
        if (!isMounted) {
          return;
        }

        setApplicationInfoState({
          status: "error",
          message: normalizeError(error),
        });
      }
    }

    async function loadDatabaseDiagnostics(): Promise<void> {
      try {
        const databaseDiagnostics = await getDatabaseDiagnostics();

        if (!isMounted) {
          return;
        }

        setDatabaseDiagnosticsState({
          status: "success",
          data: databaseDiagnostics,
        });
      } catch (error) {
        if (!isMounted) {
          return;
        }

        setDatabaseDiagnosticsState({
          status: "error",
          message: normalizeError(error),
        });
      }
    }

    void Promise.all([
      loadApplicationInfo(),
      loadDatabaseDiagnostics(),
    ]);

    return () => {
      isMounted = false;
    };
  }, []);

  function getApplicationStatus(status: string): string {
    if (status === "healthy") {
      return t("app.statusHealthy");
    }

    if (status === "ready") {
      return t("app.statusReady");
    }

    if (status === "running") {
      return t("app.statusRunning");
    }

    return formatValue(status);
  }

  function getEnvironmentLabel(environment: string): string {
    if (environment === "development") {
      return t("app.environmentDevelopment");
    }

    if (environment === "production") {
      return t("app.environmentProduction");
    }

    if (environment === "test") {
      return t("app.environmentTest");
    }

    return formatValue(environment);
  }

  function getJournalModeLabel(journalMode: string): string {
    if (journalMode.toLowerCase() === "wal") {
      return "WAL";
    }

    return formatValue(journalMode);
  }

  return (
    <main className="application-shell">
      <header className="hero-panel">
        <div className="hero-content">
          <p className="eyebrow">{t("app.title")}</p>

          <h1>{t("app.foundationTitle")}</h1>

          <p className="hero-description">
            {t("app.foundationDescription")}
          </p>
        </div>

        <div className="hero-indicator" aria-hidden="true">
          <span />
          <span />
          <span />
        </div>
      </header>

      <div className="dashboard-grid">
        <section
          className="status-panel"
          aria-labelledby="application-information-title"
          aria-live="polite"
          aria-busy={applicationInfoState.status === "loading"}
        >
          <div className="panel-heading">
            <div>
              <p className="section-label">
                {t("app.runtimeStatus")}
              </p>

              <h2 id="application-information-title">
                {t("app.applicationInformation")}
              </h2>
            </div>

            {applicationInfoState.status === "success" && (
              <span className="status-badge">
                {getApplicationStatus(
                  applicationInfoState.data.status,
                )}
              </span>
            )}
          </div>

          {applicationInfoState.status === "loading" && (
            <p className="state-message">
              {t("app.loadingApplicationInformation")}
            </p>
          )}

          {applicationInfoState.status === "error" && (
            <div className="error-message" role="alert">
              <strong>
                {t("app.applicationInformationLoadError")}
              </strong>

              <span>{applicationInfoState.message}</span>
            </div>
          )}

          {applicationInfoState.status === "success" && (
            <dl className="information-grid">
              <div>
                <dt>{t("app.name")}</dt>
                <dd>{applicationInfoState.data.name}</dd>
              </div>

              <div>
                <dt>{t("app.version")}</dt>
                <dd>{applicationInfoState.data.version}</dd>
              </div>

              <div>
                <dt>{t("app.environment")}</dt>
                <dd>
                  {getEnvironmentLabel(
                    applicationInfoState.data.environment,
                  )}
                </dd>
              </div>
            </dl>
          )}
        </section>

        <section
          className="status-panel"
          aria-labelledby="database-diagnostics-title"
          aria-live="polite"
          aria-busy={databaseDiagnosticsState.status === "loading"}
        >
          <div className="panel-heading">
            <div>
              <p className="section-label">
                {t("diagnostics.persistenceStatus")}
              </p>

              <h2 id="database-diagnostics-title">
                {t("diagnostics.databaseDiagnostics")}
              </h2>
            </div>

            {databaseDiagnosticsState.status === "success" && (
              <span
                className={`status-badge ${
                  databaseDiagnosticsState.data.reachable
                    ? ""
                    : "status-badge--error"
                }`}
              >
                {databaseDiagnosticsState.data.reachable
                  ? t("diagnostics.healthy")
                  : t("diagnostics.unreachable")}
              </span>
            )}
          </div>

          {databaseDiagnosticsState.status === "loading" && (
            <p className="state-message">
              {t("diagnostics.loadingDatabaseDiagnostics")}
            </p>
          )}

          {databaseDiagnosticsState.status === "error" && (
            <div className="error-message" role="alert">
              <strong>
                {t("diagnostics.databaseDiagnosticsLoadError")}
              </strong>

              <span>{databaseDiagnosticsState.message}</span>
            </div>
          )}

          {databaseDiagnosticsState.status === "success" && (
            <dl className="information-grid">
              <div>
                <dt>{t("diagnostics.databaseStatus")}</dt>
                <dd>
                  {databaseDiagnosticsState.data.reachable
                    ? t("diagnostics.reachable")
                    : t("diagnostics.unreachable")}
                </dd>
              </div>

              <div>
                <dt>{t("diagnostics.schemaVersion")}</dt>
                <dd>
                  {databaseDiagnosticsState.data.schemaVersion}
                </dd>
              </div>

              <div>
                <dt>{t("diagnostics.pendingMigrations")}</dt>
                <dd>
                  {databaseDiagnosticsState.data.pendingMigrations}
                </dd>
              </div>

              <div>
                <dt>{t("diagnostics.foreignKeys")}</dt>
                <dd>
                  {databaseDiagnosticsState.data.foreignKeysEnabled
                    ? t("diagnostics.enabled")
                    : t("diagnostics.disabled")}
                </dd>
              </div>

              <div>
                <dt>{t("diagnostics.journalMode")}</dt>
                <dd>
                  {getJournalModeLabel(
                    databaseDiagnosticsState.data.journalMode,
                  )}
                </dd>
              </div>

              <div className="information-grid__wide">
                <dt>{t("diagnostics.databasePath")}</dt>

                <dd
                  title={
                    databaseDiagnosticsState.data.databasePath
                  }
                >
                  {databaseDiagnosticsState.data.databasePath}
                </dd>
              </div>
            </dl>
          )}
        </section>
      </div>

      <SettingsPanel />
    </main>
  );
}

export default App;

