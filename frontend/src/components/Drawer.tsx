import React from "react";

import Box from "./Box";
import { DrawerContentComponentProps } from "@react-navigation/drawer";
import Text from "./Text";

function Drawer({ state }: DrawerContentComponentProps): JSX.Element {
  return (
    <Box>
      {/*<DrawerItems state={state} />*/}
      {state.routes.map((route) => {
        return (
          <Box>
            <Text>{route.name}</Text>
          </Box>
        );
      })}
    </Box>
  );
}

function DrawerItems({ state }: DrawerContentComponentProps) {
  return state.routes.map((route) => {
    return <Box />;
  });
}

export default Drawer;
