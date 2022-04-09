// your-app-name/src/fetchGraphQL.js
import { FetchFunction } from "relay-runtime";
import { RequestParameters } from "relay-runtime/lib/util/RelayConcreteNode";
import {
  CacheConfig,
  Variables,
} from "relay-runtime/lib/util/RelayRuntimeTypes";
import { UploadableMap } from "relay-runtime/lib/network/RelayNetworkTypes";

async function fetchGraphQL(
  request: RequestParameters,
  variables: Variables,
  cacheConfig: CacheConfig,
  uploadables?: UploadableMap | null
) {
  let b: RequestInit;
  const requestInit: {
    headers: { "Content-Type"?: string };
    method: string;
    credentials: RequestCredentials;
    body?: BodyInit;
  } = {
    method: "POST",
    credentials: "include",
    headers: {},
  };
  // Fetch data from GitHub's GraphQL API:

  // let requestVariables: {
  //   method: "POST";
  //   headers: {
  //     Accept: "application/json";
  //     "Content-Type"?: "application/json";
  //   };
  // } = {
  //   method: "POST",
  //   headers: {
  //     Accept: "application/json",
  //   },
  // };
  // let contentType = "application/json";
  // let body;
  // console.log("cool", uploadables);

  if (uploadables) {
    const formData = new FormData();
    formData.append(
      "operations",
      JSON.stringify({
        query: request.text,
        variables: variables,
      })
    );
    const uploadableMap: {
      [key: string]: string[];
    } = {};

    Object.keys(uploadables).forEach((key) => {
      if (Object.prototype.hasOwnProperty.call(uploadables, key)) {
        uploadableMap[key] = [`variables.${key}`];
      }
    });

    formData.append("map", JSON.stringify(uploadableMap));

    Object.keys(uploadables).forEach((key) => {
      if (Object.prototype.hasOwnProperty.call(uploadables, key)) {
        formData.append(key, uploadables[key]);
      }
    });

    requestInit.body = formData;
    // const data = new FormData();
    // data.append("query", request.text ?? "");
    // data.append("variables", JSON.stringify(variables));
    // data.append(
    //   "operations",
    //   JSON.stringify({
    //     query: request.text,
    //     variables,
    //   })
    // );
    // // Object.keys(uploadables).forEach((key) => {
    // //   if (Object.prototype.hasOwnProperty.call(uploadables, key)) {
    // //     data.append(key, uploadables[key]);
    // //   }
    // // });
    // data.append("map", JSON.stringify({ "0": ["variables.file"] }));
    // data.append("0", "0=@uploadables['test']");
    // body = data;
    // // console.log("my", data.);
    // contentType = "multipart/form-data";
  } else {
    requestInit.headers["Content-Type"] = "application/json";
    requestInit.body = JSON.stringify({
      query: request.text,
      variables,
    });
    // requestVariables.headers["Content-Type"] = "application/json";
    // body = JSON.stringify({
    //   query: request.text,
    //   variables,
    // });
  }

  // if (variables.hasOwnProperty("after") && variables.after !== "0") {
  //   await new Promise((res, rej) => {
  //     setTimeout(() => {
  //       res(true);
  //     }, 1500);
  //   });
  // }

  const response = await fetch("http://localhost:8000/graphql", {
    // ...requestVariables,
    ...requestInit,
    credentials: "include",
    // body,
  });

  // Get the response as JSON
  return await response.json();
}

export default fetchGraphQL;
