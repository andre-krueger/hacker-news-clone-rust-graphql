import 'react-native-gesture-handler';
import React, {useEffect, useState, Suspense} from 'react';
import {Appearance, StatusBar, useColorScheme} from 'react-native';
import {ThemeProvider} from '@shopify/restyle';
import {RelayEnvironmentProvider} from 'react-relay/hooks';

import {theme, darkTheme} from './theme';
import RelayEnvironment from './RelayEnvironment';
import Text from './components/Text';
import Navigation from './Navigation';

const App = () => {
  const [isDarkMode, setIsDarkMode] = useState(useColorScheme() === 'dark');

  useEffect(() => {
    const onChangePreferences = (preferences: {
      colorScheme: 'light' | 'dark' | null | undefined;
    }) => {
      const {colorScheme} = preferences;
      setIsDarkMode(colorScheme === 'dark');
    };
    Appearance.addChangeListener(onChangePreferences);
    return () => {
      Appearance.removeChangeListener(onChangePreferences);
    };
  }, []);

  return (
    <RelayEnvironmentProvider environment={RelayEnvironment}>
      <Suspense fallback={<Text>Loading ...</Text>}>
        <ThemeProvider theme={isDarkMode ? darkTheme : theme}>
          <StatusBar barStyle={isDarkMode ? 'light-content' : 'dark-content'} />
          <Navigation />
        </ThemeProvider>
      </Suspense>
    </RelayEnvironmentProvider>
  );
};

export default App;
