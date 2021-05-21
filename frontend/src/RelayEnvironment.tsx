import {
  Environment,
  GraphQLResponse,
  Network,
  ObservableFromValue,
  RecordSource,
  RequestParameters,
  Store,
  Variables,
} from "relay-runtime";
import Config from "react-native-config";
import { StorageService } from "./services/StorageService";
import RelayModernEnvironment from "relay-runtime/lib/store/RelayModernEnvironment";
import EnvVarNotFound from "./errors/EnvVarNotFound";

async function fetchGraphQL(
  text: RequestParameters["text"],
  variables: Variables,
  errorHandler: (error: Error) => void,
  storageService: ReturnType<typeof StorageService>
): Promise<ObservableFromValue<GraphQLResponse> | void> {
  try {
    console.log("test", Config["SCHEMA_URL"]);
    if (!Config["SCHEMA_URL"]) {
      errorHandler(new EnvVarNotFound("SCHEMA_URL not set"));
      return;
    }

    let headers: { "Content-Type": "application/json"; Cookie?: string } = {
      "Content-Type": "application/json",
    };

    const cookie = await storageService.getItem("cookie");

    if (cookie) {
      headers = { ...headers, Cookie: cookie };
    }

    const response = await fetch(Config["SCHEMA_URL"], {
      method: "POST",
      headers,
      body: JSON.stringify({
        query: text,
        variables,
      }),
    });

    const cookieHeader = response.headers.get("set-cookie");

    if (cookieHeader) {
      await storageService.setItem("cookie", cookieHeader);
    }

    return (await response.json()) as ObservableFromValue<GraphQLResponse>;
  } catch (error) {
    // errorHandler(error);
  }
}

async function fetchRelay(
  params: RequestParameters,
  variables: Variables,
  errorHandler: (error: Error) => void,
  storageService: ReturnType<typeof StorageService>
) {
  console.log(
    `Fetching query ${params.name} with ${JSON.stringify(variables)}`
  );
  return fetchGraphQL(params.text, variables, errorHandler, storageService);
}

export default (
  errorHandler: (error: Error) => void,
  storageService: ReturnType<typeof StorageService>
): RelayModernEnvironment =>
  new Environment({
    network: Network.create(
      (params, variables) =>
        fetchRelay(
          params,
          variables,
          errorHandler,
          storageService
        ) as ObservableFromValue<GraphQLResponse>
    ),
    store: new Store(new RecordSource()),
  });
