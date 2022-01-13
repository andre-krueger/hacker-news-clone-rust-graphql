import React from "react";
import ReactDOM from "react-dom";
import { App } from "./App";
import { Router, ReactLocation, useMatch, Link } from "react-location";
console.log("nst");

function Index() {
  console.log("coononu");
  return <h1>Index</h1>;
}
function Login() {
  console.log("nnnenn");
  return <h1>Login</h1>;
}
const app = document.getElementById("app");

const routes = [
  {
    path: "/",
    element: <Index />,
    // children: [
    //   {
    //     path: "/login",
    //     element: <Login />,
    //   },
    // ],
  },

  {
    path: "/login",
    element: <Login />,
  },
];

const location = new ReactLocation();
ReactDOM.render(
  <React.StrictMode>
    <Router basepath={"/admin"} routes={routes} location={location}></Router>
    <App />
  </React.StrictMode>,

  app
);
