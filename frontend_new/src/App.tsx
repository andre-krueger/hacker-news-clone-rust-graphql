import React, { Dispatch, SetStateAction, useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import {
  TestQuery,
  useDelMutation,
  useLoginMutation,
  UserEdge,
  UserResult,
  useTestQuery,
} from "./generated/graphql";
import { UseQueryState } from "urql";
import ReactPaginate from "react-paginate";

let pag = 1;

function Bla({
  numbers,
  setAfter,
}: {
  setAfter: Dispatch<SetStateAction<string | null | undefined>>;
  numbers: UseQueryState<TestQuery, object>;
}) {
  if (!numbers) {
    return <h1>test</h1>;
  }
  // if (numbers.data?.numbers.__typename) {
  switch (numbers.data?.numbers.__typename) {
    case "PaginationIncorrect": {
      return <h1>Incorrect pagination</h1>;
    }
    case "UserConnection": {
      let arr = range(1, numbers.data?.numbers.totalCount / pag + 1);
      console.log(arr);
      let b = [1, 2, 3];
      console.log(numbers.data?.numbers);
      let after = numbers.data?.numbers.pageInfo.startCursor;
      console.log(after);
      return (
        <>
          {numbers.data?.numbers.edges?.map((b) => (
            <div key={b?.node.id}>
              <h3>{b?.node.username}</h3>
              <h3>{b?.node.id}</h3>
            </div>
          ))}
          <Pagination
            items={Math.ceil(numbers.data?.numbers.totalCount / pag)}
            onClick={() => {}}
          />
          {/*{numbers.data?.numbers.pageInfo.hasNextPage && <button>Click</button>}*/}
          {/*<ReactPaginate*/}
          {/*  pageCount={Math.ceil(numbers.data?.numbers.totalCount / pag)}*/}
          {/*  onPageChange={({ selected }) => {*/}
          {/*    console.log("cool", selected * pag);*/}
          {/*    // const b = Math.ceil(parseInt("3", 10) / selected);*/}
          {/*    let t = selected * pag;*/}
          {/*    if (t === pag) {*/}
          {/*      t++;*/}
          {/*    }*/}
          {/*    setAfter(`${t}`);*/}
          {/*  }}*/}
          {/*/>*/}
        </>
      );
      // return numbers.numbers;
      // return <h1>test</h1>;
    }
    // return (
    //   <>
    //     {(numbers.data?.numbers as any).edges?.map((b: any) => (
    //       <div key={b?.node.id}>
    //         <h3>{b?.node.username}</h3>
    //         <h3>{b?.node.id}</h3>
    //       </div>
    //     ))}
    //   </>
    // );
  }
  // } else {
  //   // let x = numbers.data?.numbers as TestQuery;
  //   // x.numbers
  // }
  return null;
  // return null;
}

const range = (start: number, end: number) => {
  let length = end - start + 1;
  /*
  	Create an array of certain length and set the elements within it from
    start value to end value.
  */
  return Array.from({ length }, (_, idx) => idx + start);
};

function Pagination({
  items,
  onClick,
}: {
  items: number;
  onClick: (sel: number) => void;
}) {
  let x = Array.from({ length: items }, (_, i) => i + 1);
  console.log("bla", x);
  return (
    <ul>
      {x.map((a) => (
        <li key={`${a}`}>
          <button onClick={() => onClick(a)}>{a}</button>
        </li>
      ))}
    </ul>
  );
}

function Cool({
  setAfter,
  after,
}: {
  after: string | null | undefined;
  setAfter: Dispatch<SetStateAction<string | null | undefined>>;
}) {
  const [res, exec] = useTestQuery({ variables: { first: pag, after } });
  // Math.ceil(numbers.data?.numbers.totalCount / pag)
  return (
    <div>
      {/*<h1>{first}</h1>*/}
      {/*<button onClick={() => setFirst(++first)}>INC</button>*/}
      {/*<button onClick={() => exec({ requestPolicy: "cache-first" })}>*/}
      {/*  Refetch*/}
      {/*</button>*/}

      <Bla numbers={res} setAfter={setAfter} />
    </div>
  );
}

function App() {
  // const [res, exec] = useTestQuery({ variables: { first: 3 } });
  // res.data?.numbers.__typename==='PaginationIncorrect';
  // res.data;
  // console.log(res);
  const [after, setAfter] = useState<string | null | undefined>(null);
  const [res, ex] = useLoginMutation();
  const [res_, exec] = useDelMutation();
  // console.log("cool", ex());
  useEffect(() => {
    console.log("aber", res_);
  }, [res_]);

  const [pageVariables, setPageVariables] = useState([
    {
      first: 10,
      after: null,
    },
  ]);

  return (
    <div className="App">
      <header className="App-header">
        <button
          onClick={() => {
            ex({ username: "admin", password: "admin" });
          }}
        >
          login
        </button>
        <button
          onClick={() => {
            exec();
          }}
        >
          Del
        </button>
        {/*<button onClick={() => exec( )}>Refetch</button>*/}
        <Cool setAfter={setAfter} after={after} />
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
