export type ThemePreference = "system" | "light" | "dark";

export type LanguagePreference = "en" | "pt-BR";

export interface ApplicationSettings {
  theme: ThemePreference;
  language: LanguagePreference;
}

export interface UpdateApplicationSettingsInput {
  theme: ThemePreference;
  language: LanguagePreference;
}