import Config from 'react-native-config';
import {Environment, Network, RecordSource, Store} from 'relay-runtime';
import AsyncStorage from '@react-native-async-storage/async-storage';
import CookieManager from '@react-native-cookies/cookies';

async function fetchGraphQL(text, variables) {
  console.log('test', Config.SCHEMA_URL);
  console.log('bla', await AsyncStorage.getItem('cookie'));
  try {
    const response = await fetch(Config.SCHEMA_URL, {
      method: 'POST',
      credentials: 'omit',
      headers: {
        // Authorization: `beare`,
        'Content-Type': 'application/json',
        // Cookie:
        //   'sid=Y55ibZqp+UqQpctBLgxm+v0D6yD/+c7Q6mopQlhlcVPfgCj4IfLMpLlaHSMIRDmDOfnG1lMfgnGTdmt8VTX0nA==',
      },
      body: JSON.stringify({
        query: text,
        variables,
      }),
    });
    CookieManager.get('http://localhost:8000/graphql').then(n => {
      console.log('enut', n);
    });
    CookieManager.setFromResponse('http://localhost:8000/graphql', '').then(
      tb => {
        console.log('nnnninern', tb);
      },
    );
    console.log(response.headers.get('set-cookie'));
    const t = await response.json();

    return t;
  } catch (e) {
    // TODO: Display error to user via relay local state error store
    console.log(e);
  }
}

async function fetchRelay(params, variables) {
  return fetchGraphQL(params.text, variables);
}

export default new Environment({
  network: Network.create(fetchRelay),
  store: new Store(new RecordSource()),
});
