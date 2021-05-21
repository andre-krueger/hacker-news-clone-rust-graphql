import "react-i18next";
import ns from "./translations/en.json";

declare module "react-i18next" {
  interface Resources {
    ns: typeof ns;
  }
}
