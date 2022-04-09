module.exports = {
  presets: ["module:metro-react-native-babel-preset"],
  plugins: [
    "transform-inline-environment-variables",
    "relay",
    "react-native-reanimated/plugin",
  ],
};
