import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { createClient, dedupExchange, fetchExchange, Provider } from "urql";
import { cacheExchange } from "@urql/exchange-graphcache";
import { relayPagination } from "@urql/exchange-graphcache/extras";
import schema from "./generated/schema";

// const cache = cacheExchange({
//   resolvers: { Query: { numbers: relayPagination() } },
// });
const cache = cacheExchange({
  keys: { PaginationIncorrect: (data) => null },
  schema,
  resolvers: { Query: { numbers: relayPagination() } },
});

const client = createClient({
  url: "http://localhost:8000/graphql",
  fetchOptions: () => {
    return { credentials: "include" };
  },
  //
  exchanges: [dedupExchange, cache, fetchExchange],
  //     exchanges:[
  //         {
  // input:"",
  //             cacheExchange({resolvers:{Query:{numbers: relayPagination()}}})
  //         }
  //     ]
});

ReactDOM.render(
  <React.StrictMode>
    <Provider value={client}>
      <App />
    </Provider>
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
