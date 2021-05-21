import React, { useCallback, useEffect, useMemo, useState } from "react";
import { Appearance, useColorScheme } from "react-native";
import { RelayEnvironmentProvider } from "react-relay";
import { ThemeProvider } from "@shopify/restyle";
import { useMachine as xstateUseMachine } from "@xstate/react";
import RelayEnvironment from "./RelayEnvironment";
import { darkTheme, theme } from "./Theme";
import Navigation from "./navigation/Navigation";
import { StorageService } from "./services/StorageService";
import { errorMachine } from "./machines/ErrorMachine";
import ErrorModal from "./modals/ErrorModal";
import { MachinesContextProvider, useMachine } from "./machines";
import {
  ApolloClient,
  ApolloProvider,
  createHttpLink,
  InMemoryCache,
} from "@apollo/client";
import LoginScreen from "./screens/LoginScreen";
const cookie = "";
const link = createHttpLink({
  uri: "http://localhost:8000/graphql",
  credentials: "include",
  // fetch: async (input: RequestInfo, test: RequestInit) => {
  //   // test = { ...test, head } };
  //   // test.headers = { ...test.headers, Cookie: cookie };
  //   console.log("tst", test);
  //   const t = await fetch(input, test);
  //   console.log("bbbbb", t);
  //   // cookie = t.headers.get("set-cookie");
  //   return t;
  // },
});

const client = new ApolloClient({
  cache: new InMemoryCache(),
  link,
});

function AppWrapper(): JSX.Element {
  const errorMachineValue = xstateUseMachine(errorMachine);
  const value = useMemo(() => ({ errorMachine: errorMachineValue }), [
    errorMachineValue,
  ]);
  return (
    <MachinesContextProvider value={value}>
      <App />
    </MachinesContextProvider>
  );
}

function App() {
  const { isDarkMode } = useHandleChangePreferences();
  const [visible, setIsVisible] = useState(false);
  const [state, send] = useMachine().errorMachine;
  const errorHandler = useCallback(
    (error: Error) => {
      send("SET_ERROR", error);
    },
    [send]
  );

  useEffect(() => {
    setIsVisible(state.matches("error"));
  }, [state]);

  return (
    <RelayEnvironmentProvider
      environment={RelayEnvironment(errorHandler, StorageService(errorHandler))}
    >
      {/*<ApolloProvider client={client}>*/}
      <ThemeProvider theme={isDarkMode ? darkTheme : theme}>
        <ErrorModal
          visible={visible}
          error={state.context.error}
          resetError={() => send("RESET")}
        />
        <LoginScreen />
        {/*<Navigation />*/}
      </ThemeProvider>
      {/*</ApolloProvider>*/}
    </RelayEnvironmentProvider>
  );
}

function useHandleChangePreferences() {
  const [isDarkMode, setIsDarkMode] = useState(useColorScheme() === "dark");
  console.log("test");
  useEffect(() => {
    const onChangePreferences = (preferences: {
      colorScheme: "light" | "dark" | null | undefined;
    }) => {
      const { colorScheme } = preferences;
      console.log(colorScheme);
      setIsDarkMode(colorScheme === "dark");
    };
    Appearance.addChangeListener(onChangePreferences);
    return () => {
      Appearance.removeChangeListener(onChangePreferences);
    };
  }, []);
  return { isDarkMode };
}

export default AppWrapper;
