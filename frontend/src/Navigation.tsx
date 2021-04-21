import React from 'react';
import {DrawerActions, NavigationContainer} from '@react-navigation/native';
import {createStackNavigator} from '@react-navigation/stack';
import LoginPage from './components/LoginPage';
import HomePage from './components/HomePage';
import Box from './components/Box';
import Text from './components/Text';
import Log from './Log';
import {createDrawerNavigator} from '@react-navigation/drawer';
import {Button, View} from 'react-native';
import ArticlePage from './components/ArticlePage';
import NavigationBar from './components/NavigationBar';

const Drawer = createDrawerNavigator();
const Stack = createStackNavigator();

function HomeStack() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        gestureEnabled: true,
      }}>
      <Stack.Screen name={'HomePage'} component={HomePage} />
      <Stack.Screen name={'ArticlePage'} component={ArticlePage} />
    </Stack.Navigator>
  );
}

function Navigation() {
  return (
    <NavigationContainer>
      <Drawer.Navigator
        screenOptions={{
          swipeEnabled: false,
          headerShown: true,
          header: () => <NavigationBar />,
        }}>
        <Drawer.Screen name={'HomeStack'} component={HomeStack} />
        <Drawer.Screen name={'LoginPage'} component={LoginPage} />
      </Drawer.Navigator>
    </NavigationContainer>
  );
}

export default Navigation;
