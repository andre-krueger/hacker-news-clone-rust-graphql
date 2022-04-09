import React from "react";
import HomeScreen from "../screens/HomeScreen";
import ArticleScreen from "../screens/ArticleScreen";
import { createStackNavigator } from "@react-navigation/stack";

export type HomeStackParamList = {
  HomeScreen: undefined;
  ArticleScreen: undefined;
};

const Stack = createStackNavigator<HomeStackParamList>();

function HomeStack(): JSX.Element {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        gestureEnabled: true,
      }}
    >
      <Stack.Screen name={"HomeScreen"} component={HomeScreen} />
      <Stack.Screen name={"ArticleScreen"} component={ArticleScreen} />
    </Stack.Navigator>
  );
}

export default HomeStack;
