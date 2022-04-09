import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { createClient, dedupExchange, fetchExchange, Provider } from "urql";
import { cacheExchange } from "@urql/exchange-graphcache";

import { BrowserRouter, Routes, Route } from "react-router-dom";
import {
  simplePagination,
  relayPagination,
} from "@urql/exchange-graphcache/extras";
import schema from "./generated/schema";
import { multipartFetchExchange } from "@urql/exchange-multipart-fetch";

// const cache = cacheExchange({
//   resolvers: { Query: { numbers: relayPagination() } },
// });
const cache = cacheExchange({
  // schema,
  resolvers: {
    Query: {
      numbers: relayPagination(),
      // numbers2: simplePagination({
      //   limitArgument: "limit",
      //   offsetArgument: "skip",
      //   mergeMode: "after",
      // }),
    },
  },
});

const client = createClient({
  url: "http://localhost:8000/graphql",
  fetchOptions: () => {
    return { credentials: "include" };
  },
  //
  exchanges: [dedupExchange, cache, multipartFetchExchange],
  //     exchanges:[
  //         {
  // input:"",
  //             cacheExchange({resolvers:{Query:{numbers: relayPagination()}}})
  //         }
  //     ]
});

const Bla = () => {
  return <h1>bla</h1>;
};

ReactDOM.render(
  <React.StrictMode>
    <Provider value={client}>
      <BrowserRouter>
        <Routes>
          <Route path={"/"} element={<App />} />
          <Route path={"/test"} element={<Bla />} />
          {/*<App />*/}
        </Routes>
      </BrowserRouter>
    </Provider>
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
