import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type PropsWithChildren,
} from "react";

import {
  translations,
  type SupportedLanguage,
} from "./translations";

interface I18nContextValue {
  language: SupportedLanguage;
  setLanguage: (language: SupportedLanguage) => void;
  t: (key: string) => string;
}

const I18nContext = createContext<I18nContextValue | null>(null);

interface I18nProviderProps extends PropsWithChildren {
  initialLanguage?: SupportedLanguage;
}

export function I18nProvider({
  children,
  initialLanguage = "en",
}: I18nProviderProps) {
  const [language, setLanguage] =
    useState<SupportedLanguage>(initialLanguage);

  const t = useCallback(
    (key: string): string => {
      const parts = key.split(".");

      let value: unknown = translations[language];

      for (const part of parts) {
        if (
          typeof value !== "object" ||
          value === null ||
          !(part in value)
        ) {
          return key;
        }

        value = (value as Record<string, unknown>)[part];
      }

      return typeof value === "string"
        ? value
        : key;
    },
    [language],
  );

  const context = useMemo<I18nContextValue>(
    () => ({
      language,
      setLanguage,
      t,
    }),
    [language, t],
  );

  return (
    <I18nContext.Provider value={context}>
      {children}
    </I18nContext.Provider>
  );
}

export function useI18n(): I18nContextValue {
  const context = useContext(I18nContext);

  if (context === null) {
    throw new Error(
      "useI18n must be used within an I18nProvider.",
    );
  }

  return context;
}