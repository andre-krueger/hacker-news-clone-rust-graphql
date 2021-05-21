import { action } from "@storybook/addon-actions";
import { text } from "@storybook/addon-knobs";
import { storiesOf } from "@storybook/react-native";
import React from "react";
import Button from "../../../src/components/Button";
import CenterView from "../CenterView";

storiesOf("Button", module)
  .addDecorator((getStory) => <CenterView>{getStory()}</CenterView>)
  .add("with text", () => (
    <Button
      title={text("Button text", "Hello button")}
      onPress={() => {
        console.log("cool");
      }}
    />
  ))
  .add("with some emoji", () => (
    <Button
      title={text("Button text", "😀 😎 👍 💯")}
      onPress={action("clicked-emoji")}
    />
  ));
