import React from "react";
import { Pressable } from "react-native";
import Text from "./Text";
import Box from "./Box";

type Props = { title: string; onPress: (...args: unknown[]) => void };

function Button({ title, onPress }: Props): JSX.Element {
  return (
    <Pressable onPress={onPress}>
      <Box
        backgroundColor={"primaryButtonBackground"}
        width={130}
        height={40}
        justifyContent={"center"}
        alignItems={"center"}
        padding={"s"}
      >
        <Text adjustsFontSizeToFit numberOfLines={1} variant={"button"}>
          {title}
        </Text>
      </Box>
    </Pressable>
  );
}

export default Button;
