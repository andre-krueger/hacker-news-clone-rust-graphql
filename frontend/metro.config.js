module.exports = {
  transformer: {
    getTransformOptions: () => ({
      transform: {
        experimentalImportSupport: false,
        // Set to false to make Storybook work
        // See: https://github.com/storybookjs/react-native/issues/152
        inlineRequires: false,
      },
    }),
  },
};
