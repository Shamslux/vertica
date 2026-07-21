export const translations = {
  en: {
    app: {
      title: "Vertica",
      subtitle: "Desktop application powered by Tauri, React and Rust.",
    },

    dashboard: {
      title: "Dashboard",
      description: "Everything is running correctly.",
    },

    diagnostics: {
      title: "Diagnostics",
      database: "Database",
      schemaVersion: "Schema version",
      pendingMigrations: "Pending migrations",
      foreignKeys: "Foreign keys",
      journalMode: "Journal mode",
      enabled: "Enabled",
      disabled: "Disabled",
    },

    settings: {
      title: "Settings",
      description: "Customize your Vertica experience.",

      preferences: "Preferences",
      applicationSettings: "Application settings",

      theme: "Theme",
      themeDescription:
        "Choose how Vertica should appear on this device.",

      language: "Language",
      languageDescription:
        "Choose the language used throughout the application.",

      system: "System",
      light: "Light",
      dark: "Dark",

      english: "English",
      portugueseBrazil: "Português (Brasil)",

      storage: "Storage",
      activeProfileDatabase: "Active profile database",
      persistent: "Persistent",

      changesSavedAutomatically:
        "Changes are saved automatically.",

      loading: "Loading",
      loadingSettings: "Loading application settings...",
      saving: "Saving...",
      saved: "Saved",
      failed: "Failed",

      loadError: "Unable to load application settings.",
      saveError: "Unable to save application settings.",
      unexpectedError:
        "An unexpected error occurred while processing settings.",
    },
  },

  "pt-BR": {
    app: {
      title: "Vertica",
      subtitle:
        "Aplicação desktop desenvolvida com Tauri, React e Rust.",
    },

    dashboard: {
      title: "Painel",
      description: "Tudo está funcionando corretamente.",
    },

    diagnostics: {
      title: "Diagnóstico",
      database: "Banco de dados",
      schemaVersion: "Versão do esquema",
      pendingMigrations: "Migrações pendentes",
      foreignKeys: "Chaves estrangeiras",
      journalMode: "Modo do journal",
      enabled: "Ativado",
      disabled: "Desativado",
    },

    settings: {
      title: "Configurações",
      description: "Personalize sua experiência no Vertica.",

      preferences: "Preferências",
      applicationSettings: "Configurações do aplicativo",

      theme: "Tema",
      themeDescription:
        "Escolha como o Vertica deve aparecer neste dispositivo.",

      language: "Idioma",
      languageDescription:
        "Escolha o idioma utilizado em todo o aplicativo.",

      system: "Sistema",
      light: "Claro",
      dark: "Escuro",

      english: "English",
      portugueseBrazil: "Português (Brasil)",

      storage: "Armazenamento",
      activeProfileDatabase: "Banco de dados do perfil ativo",
      persistent: "Persistente",

      changesSavedAutomatically:
        "As alterações são salvas automaticamente.",

      loading: "Carregando",
      loadingSettings:
        "Carregando as configurações do aplicativo...",
      saving: "Salvando...",
      saved: "Salvo",
      failed: "Falhou",

      loadError:
        "Não foi possível carregar as configurações do aplicativo.",
      saveError:
        "Não foi possível salvar as configurações do aplicativo.",
      unexpectedError:
        "Ocorreu um erro inesperado ao processar as configurações.",
    },
  },
} as const;

export type SupportedLanguage = keyof typeof translations;

export type TranslationDictionary =
  (typeof translations)[SupportedLanguage];

