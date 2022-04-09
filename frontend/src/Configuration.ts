import Config from "react-native-config";

const withStorybook = Config["WITH_STORYBOOK"] === "true";

const startWithStorybook = process.env["START_WITH_STORYBOOK"] === "true";

const configuration = {
  withStorybook,
  startWithStorybook,
};

export default configuration;
