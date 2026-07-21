use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use super::settings_repository::SettingsRepositoryError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemePreference {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl TryFrom<&str> for ThemePreference {
    type Error = SettingsValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "system" => Ok(Self::System),
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(SettingsValidationError::InvalidTheme(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationSettings {
    pub theme: ThemePreference,
    pub language: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateApplicationSettings {
    pub theme: ThemePreference,
    pub language: String,
}

impl UpdateApplicationSettings {
    pub fn new(
        theme: ThemePreference,
        language: impl Into<String>,
    ) -> Result<Self, SettingsRepositoryError> {
        let language = language.into();

        validate_language(&language)
            .map_err(|error| SettingsRepositoryError::new(error.to_string()))?;

        Ok(Self { theme, language })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsValidationError {
    InvalidTheme(String),
    InvalidLanguage(String),
}

impl fmt::Display for SettingsValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTheme(value) => {
                write!(formatter, "unsupported theme preference: {value}")
            }
            Self::InvalidLanguage(value) => {
                write!(formatter, "unsupported language preference: {value}")
            }
        }
    }
}

impl Error for SettingsValidationError {}

pub fn validate_language(language: &str) -> Result<(), SettingsValidationError> {
    match language {
        "en" | "pt-BR" => Ok(()),
        _ => Err(SettingsValidationError::InvalidLanguage(
            language.to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_language, ThemePreference, UpdateApplicationSettings};

    #[test]
    fn parses_supported_themes() {
        assert_eq!(
            ThemePreference::try_from("system"),
            Ok(ThemePreference::System),
        );

        assert_eq!(
            ThemePreference::try_from("light"),
            Ok(ThemePreference::Light),
        );

        assert_eq!(ThemePreference::try_from("dark"), Ok(ThemePreference::Dark),);
    }

    #[test]
    fn rejects_unsupported_theme() {
        assert!(ThemePreference::try_from("blue").is_err());
    }

    #[test]
    fn accepts_supported_languages() {
        assert!(validate_language("en").is_ok());
        assert!(validate_language("pt-BR").is_ok());
    }

    #[test]
    fn rejects_unsupported_languages() {
        assert!(validate_language("pt").is_err());
        assert!(validate_language("pt-br").is_err());
        assert!(validate_language("es").is_err());
    }

    #[test]
    fn creates_valid_update_settings() {
        let settings = UpdateApplicationSettings::new(ThemePreference::Dark, "pt-BR")
            .expect("supported settings should be accepted");

        assert_eq!(settings.theme, ThemePreference::Dark);
        assert_eq!(settings.language, "pt-BR");
    }
}
