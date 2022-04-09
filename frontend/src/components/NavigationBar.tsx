import React from "react";
import { Pressable } from "react-native";
import { DrawerActions, useNavigation } from "@react-navigation/native";
import { useTheme } from "@shopify/restyle";
import Box from "./Box";
import { Theme } from "../Theme";
import Icon from "./Icon";

function NavigationBar(): JSX.Element {
  const navigation = useNavigation();
  const theme = useTheme<Theme>();
  return (
    <Box
      flexBasis={"10%"}
      // alignItems={'center'}
      padding={"s"}
      backgroundColor={"secondaryBackground"}
    >
      <Box flex={1} alignItems={"center"} flexDirection={"row"}>
        <Pressable
          android_ripple={{
            color: theme.colors.mainForeground,
            borderless: true,
          }}
          onPress={() => navigation.dispatch(() => DrawerActions.openDrawer())}
        >
          <Icon name={"menu"} size={32} />
        </Pressable>
      </Box>
    </Box>
  );
}

export default NavigationBar;
