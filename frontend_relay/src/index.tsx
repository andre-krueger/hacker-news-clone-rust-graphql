import React, { Suspense } from "react";
import ReactDOM from "react-dom";
import "./index.css";

import Bla from "./Bla";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Router, ReactLocation, useMatch, Link } from "react-location";
import { parseSearch, stringifySearch } from "react-location-jsurl";

import {
  loadQuery,
  RelayEnvironmentProvider,
  usePreloadedQuery,
} from "react-relay/hooks";
import RelayEnvironment from "./RelayEnvironment";
import graphql from "babel-plugin-relay/macro";
import { useMutation, usePaginationFragment } from "react-relay";
import srcindexQueryGraphql from "./__generated__/srcindexQuery.graphql";

export const deleteMutation = graphql`
  mutation srcindexDeleteMutation {
    deleteUser(id: 1)
  }
`;
const feedbackLikeMutation = graphql`
  mutation srcindexMutation {
    login(username: "andre", password: "andre") {
      username
    }
  }
`;

const appFragment = graphql`
  fragment srcindexFragment on QueryRoot
  @refetchable(queryName: "srcindexPaginationQuery") {
    numbers(
      first: $first
      after: $after
      before: $before
      last: $last
      filter: $filter
      skip: $skip
    ) @connection(key: "srcindexFragment_numbers") {
      totalCount
      paginationVec
      edges {
        node {
          id
          username
        }
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }
`;

function About() {
  const {
    data: { bla },
  } = useMatch() as any;

  const [commit] = useMutation(feedbackLikeMutation);
  const [commit2] = useMutation(deleteMutation);

  const _data = usePreloadedQuery(
    graphql`
      query srcindexQuery(
        $first: Int
        $after: String
        $before: String
        $last: Int
        $filter: UsersFilterInput
        $skip: Int
      ) {
        ...srcindexFragment
      }
    `,
    bla
  );

  const { data, loadNext, loadPrevious, refetch, isLoadingNext } =
    usePaginationFragment(appFragment, _data as any);
  console.log(data);
  return (
    <>
      <h1>test</h1>
      <button onClick={() => commit({ variables: {} })}>login</button>
      <button
        onClick={() =>
          commit2({
            variables: {},
            onCompleted() {
              console.log("done");
            },
          })
        }
      >
        delete
      </button>
    </>
  );
}

const routes = [
  {
    path: "/",
    element: (
      <Link preload={1} to={"/admin"}>
        about
      </Link>
    ),
  },
  {
    path: "admin",
    loader: () => ({
      // bla: await fetch("http://localhost:3000"),
      bla: loadQuery(
        RelayEnvironment,
        srcindexQueryGraphql,
        // graphql`
        //   query srcindexQuery(
        //     $first: Int
        //     $after: String
        //     $before: String
        //     $last: Int
        //     $filter: UsersFilterInput
        //     $skip: Int
        //   ) {
        //     ...srcindexFragment
        //   }
        // `,
        { first: 1 },
        { fetchPolicy: "store-or-network" }
      ),
    }),

    element: (
      <Suspense fallback={"loading"}>
        <About />
      </Suspense>
    ),
  },
];
const location = new ReactLocation({ parseSearch, stringifySearch });
ReactDOM.render(
  <React.StrictMode>
    <RelayEnvironmentProvider environment={RelayEnvironment}>
      <Router
        // defaultLinkPreloadMaxAge={500}
        location={location}
        routes={routes}
      ></Router>
    </RelayEnvironmentProvider>
  </React.StrictMode>,
  document.getElementById("root")
);
// ReactDOM.render(
//   <React.StrictMode>
//     <RelayEnvironmentProvider environment={RelayEnvironment}>
//       <BrowserRouter basename={"/admin"}>
//         <Routes>
//           <Route path={"/"} element={<App />} />
//           <Route path={"/test"} element={<Bla />} />
//           {/*<App />*/}
//         </Routes>
//       </BrowserRouter>
//     </RelayEnvironmentProvider>
//   </React.StrictMode>,
//   document.getElementById("root")
// );

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
