import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { TestQuery, UserResult, useTestQuery } from "./generated/graphql";
import { UseQueryState } from "urql";

function Bla({ numbers }: { numbers: UseQueryState<TestQuery, object> }) {
  if (!numbers) {
    return <h1>test</h1>;
  }
  switch (numbers.data?.numbers.__typename) {
    case "PaginationIncorrect": {
      return <h1>Incorrect pagination</h1>;
    }
    case "UserConnection":
      {
        return (
          <>
            {numbers.data?.numbers.edges?.map((b) => (
              <div key={b?.node.id}>
                <h3>{b?.node.username}</h3>
                <h3>{b?.node.id}</h3>
              </div>
            ))}
          </>
        );
        // return numbers.numbers;
        // return <h1>test</h1>;
      }
      break;
    default:
      break;
  }
  return null;
  // return null;
}

function App() {
  // const [res, exec] = useTestQuery({ variables: { first: 3 } });
  const [res, exec] = useTestQuery({ variables: { first: 2, after: "2" } });
  // res.data?.numbers.__typename==='PaginationIncorrect';
  // res.data;
  // console.log(res);

  return (
    <div className="App">
      <header className="App-header">
        <Bla numbers={res} />
        {/*<Bl numbers={res}/>*/}
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
