// your-app-name/src/RelayEnvironment.js
import { Environment, Network, RecordSource, Store } from "relay-runtime";
import fetchGraphQL from "./fetchGraphQL";
import { RequestParameters } from "relay-runtime/lib/util/RelayConcreteNode";
import {
  CacheConfig,
  Variables,
} from "relay-runtime/lib/util/RelayRuntimeTypes";
import { UploadableMap } from "relay-runtime/lib/network/RelayNetworkTypes";

// Export a singleton instance of Relay Environment configured with our network function:
export default new Environment({
  network: Network.create(fetchGraphQL),
  store: new Store(new RecordSource()),
});
