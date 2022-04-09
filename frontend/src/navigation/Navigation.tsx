import React, { Suspense } from "react";
import { DefaultTheme, NavigationContainer } from "@react-navigation/native";
import { createDrawerNavigator } from "@react-navigation/drawer";
import NavigationBar from "../components/NavigationBar";
import LoginScreen from "../screens/LoginScreen";
import HomeStack from "./HomeStack";
import configuration from "../Configuration";
import StoryBook from "../../storybook/index";
import { useTheme } from "@shopify/restyle";
import { Theme } from "../Theme";
import DrawerComponent from "../components/Drawer";
import { graphql, useQueryLoader } from "react-relay";

type DrawerStackParamList = {
  HomeStack: undefined;
  StoryBook: undefined;
  LoginPage: undefined;
};

const Drawer = createDrawerNavigator<DrawerStackParamList>();

const query = graphql`
  query NavigationQuery {
    user(id: 1) {
      id
      role
    }
  }
`;

function Navigation(): JSX.Element {
  const theme = useTheme<Theme>();
  return (
    <NavigationContainer
      theme={{
        ...DefaultTheme,
        colors: {
          ...DefaultTheme.colors,
          background: theme.colors.primaryBackground,
          card: theme.colors.primaryBackground,
        },
      }}
    >
      <Drawer.Navigator
        screenOptions={{
          swipeEnabled: false,
          headerShown: true,
          header: () => <NavigationBar />,
        }}
        // drawerContent={({ state }) => <DrawerComponent state={state} />}
        initialRouteName={
          configuration.withStorybook && configuration.startWithStorybook
            ? "StoryBook"
            : "HomeStack"
        }
      >
        <Drawer.Screen name={"HomeStack"} component={HomeStack} />
        <Drawer.Screen
          name={"LoginPage"}
          component={(props) => (
            <Suspense fallback={null}>
              <LoginScreen props={props} />
            </Suspense>
          )}
        />
        {configuration.withStorybook && (
          <Drawer.Screen name={"StoryBook"} component={StoryBook} />
        )}
      </Drawer.Navigator>
    </NavigationContainer>
  );
}

export default Navigation;
