import { createTheme } from "@shopify/restyle";

export const palette = {
  purple: "#5A31F4",
  white: "#FFF",
  black: "#111",
  orange: "#ff6600",
  lighterOrange: "#FFAC0D",
  marshmallow: "#f6f6ef",
  darkGray: "#333",
  lightGray: "#EEE",
};

export const theme = createTheme({
  spacing: {
    s: 8,
    m: 16,
  },
  colors: {
    mainBackground: palette.lightGray,
    mainForeground: palette.black,
    primaryBackground: palette.marshmallow,
    secondaryBackground: palette.orange,
    primaryCardBackground: palette.purple,
    secondaryCardBackground: palette.white,
    primaryCardText: palette.white,
    secondaryCardText: palette.black,
    primaryButtonBackground: palette.lighterOrange,
    primaryButtonText: palette.black,
  },
  breakpoints: {},
  textVariants: {
    body: {
      fontSize: 16,
      lineHeight: 20,
      color: "mainForeground",
    },
    button: {
      fontSize: 16,
      lineHeight: 20,
      color: "primaryButtonText",
    },
    icon: {
      fontSize: 32,
    },
  },
  cardVariants: {
    primary: {
      backgroundColor: "primaryCardBackground",
      shadowOpacity: 0.3,
    },
    secondary: {
      backgroundColor: "secondaryCardBackground",
      shadowOpacity: 0.1,
    },
  },
});

export type Theme = typeof theme;

export const darkTheme: Theme = {
  ...theme,
  colors: {
    ...theme.colors,
    mainBackground: palette.black,
    mainForeground: palette.white,
    primaryBackground: palette.black,
    secondaryBackground: palette.orange,

    secondaryCardBackground: palette.darkGray,
    secondaryCardText: palette.white,
    primaryButtonText: palette.black,
  },
};
