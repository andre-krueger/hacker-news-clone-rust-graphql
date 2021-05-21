import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./translations/en.json";

i18n
  .use(initReactI18next)
  .init({
    resources: { en },
    lng: "en",
    ns: ["en"],
    fallbackLng: "en",
    interpolation: { escapeValue: false },
  })
  .catch(() => {});

export default i18n;
