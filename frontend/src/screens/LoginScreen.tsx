import React, { Component, Suspense, useCallback, useEffect } from "react";
import { Button, Text, View } from "react-native";
import { useFocusEffect, useRoute } from "@react-navigation/native";
import { graphql, usePreloadedQuery, useQueryLoader } from "react-relay";
import { gql } from "@apollo/client/core";
import { useMutation, useQuery } from "@apollo/client";
import { LoginScreenQuery } from "./__generated__/LoginScreenQuery.graphql";

class ErrorBoundary extends Component<any, any> {
  constructor(props) {
    super(props);
    this.state = { hasError: false, error: undefined };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true, error: error };
  }

  render() {
    if (this.state.error) {
      console.log("aber", this.state.error.stack);
      console.log("aber2", this.state.error.message);
      console.log("aber3", JSON.stringify(this.state.error.source));
    }
    if (this.state.hasError) {
      return (
        <View>
          <Button
            title={"test"}
            onPress={() => {
              console.log("test");
            }}
          />
        </View>
      );
    }
    return this.props.children;
  }
}

const AppQuery = graphql`
  query LoginScreenQuery($id: ID!) {
    user(id: $id) {
      id
      role
    }
  }
`;

const AppQuery2 = gql`
  mutation Login {
    login(username: "test", password: "test") {
      id
    }
  }
`;
const AppQuery3 = gql`
  mutation Delete {
    deleteUser(id: 1)
  }
`;
function LoginScreen(): JSX.Element {
  // const t = useRoute();
  // console.log("bla", t.params);
  //
  // const { loading, data, error } = useQuery(AppQuery);
  // const [login] = useMutation(AppQuery2);
  //
  // const [deleteUser] = useMutation(AppQuery3);
  // console.log(data);
  const [queryReference, load, dispose] = useQueryLoader<LoginScreenQuery>(
    AppQuery
  );
  // useFocusEffect(
  //   useCallback(() => {
  //     console.log("nn", queryReference);
  //     load({ id: "10" });
  //     return () => {
  //       dispose();
  //     };
  //   }, [])
  // );
  useEffect(() => {
    console.log("nn", queryReference);
    load({ id: "10" });
    return () => {
      dispose();
    };
  }, []);
  // console.log("rnt", qref);
  // // useEffect(() => {
  // //   load({});
  // // }, []);
  // if (qref !== null) {
  //   return <Test qref={qref} />;
  // }
  console.log("st", queryReference);
  return (
    <ErrorBoundary fallback={() => null}>
      <Suspense fallback={null}>
        <View style={{ flex: 1, backgroundColor: "red" }}>
          <Button
            title={"cool"}
            onPress={() => {
              load({ id: 1 });
            }}
          />
          {/*<Button*/}
          {/*  title={"login"}*/}
          {/*  onPress={() => {*/}
          {/*    login();*/}
          {/*  }}*/}
          {/*/>*/}
          {/*<Button*/}
          {/*  title={"delete"}*/}
          {/*  onPress={() => {*/}
          {/*    deleteUser();*/}
          {/*  }}*/}
          {/*/>*/}
          {queryReference && <Test qref={queryReference} />}
        </View>
      </Suspense>
    </ErrorBoundary>
  );
}
//
function Test({ qref }) {
  const data = usePreloadedQuery<LoginScreenQuery>(AppQuery, qref);
  console.log("test", data);

  return (
    <Suspense fallback={null}>
      <Text>{data.user.id}</Text>
      <Text>{data.user.role}</Text>
    </Suspense>
  );
}

export default LoginScreen;
