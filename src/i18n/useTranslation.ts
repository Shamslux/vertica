import { useI18n } from "./I18nProvider";

export function useTranslation() {
  const { t, language, setLanguage } = useI18n();

  return {
    t,
    language,
    setLanguage,
  };
}