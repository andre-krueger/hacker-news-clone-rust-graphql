import React, { Suspense, useEffect, useMemo, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import graphql from "babel-plugin-relay/macro";
import {
  useLazyLoadQuery,
  useMutation,
  usePaginationFragment,
} from "react-relay";
import AppFragmentGraphql, {
  AppFragment,
  AppFragment$key,
} from "./__generated__/AppFragment.graphql";
import { AppQuery, UsersFilterInput } from "./__generated__/AppQuery.graphql";
import appMutationGraphql from "./__generated__/AppMutation.graphql";
import { blafeedbackLikeMutation } from "./Login";
import Upload from "./Upload";
import { useTable, usePagination } from "react-table";

const feedbackLikeMutation = graphql`
  mutation AppMutation {
    login(username: "andre", password: "andre") {
      username
    }
  }
`;

const appFragment = graphql`
  fragment AppFragment on QueryRoot
  @refetchable(queryName: "AppPaginationQuery") {
    numbers(
      first: $first
      after: $after
      before: $before
      last: $last
      filter: $filter
    ) @connection(key: "AppFragment_numbers") {
      totalCount
      edges {
        node {
          id
          username
        }
      }
    }
  }
`;

function Cool({ bla }: { bla: AppFragment$key }) {
  const { data, loadNext, loadPrevious } = usePaginationFragment(
    appFragment,
    bla
  );
  const [commit] = useMutation(feedbackLikeMutation);
  const [commit2] = useMutation(blafeedbackLikeMutation);

  const data2 = useMemo(() => [{ col1: "Hello" }, { col1: "Cool" }], []) as any;

  const columns = useMemo(
    () => [
      { Header: "Id", accessor: "node.id" },
      { Header: "USername", accessor: "node.username" },
    ],
    []
  );

  console.log("cool3", data.numbers.edges);
  const instance = useTable(
    {
      columns,
      data: data.numbers.edges as any,
      // initialState: { pageSize: 2 },
      manualPagination: true,
      pageCount: 5,
    } as any,
    usePagination
  );
  const {
    getTableProps,

    getTableBodyProps,

    headerGroups,

    rows,

    prepareRow,
    pageOptions,
    page,
    state: { pageIndex, pageSize },
    gotoPage,
    previousPage,
    nextPage,
    setPageSize,
    canPreviousPage,
    canNextPage,
  } = instance as any;
  return (
    <>
      {/*<table {...getTableProps()}>*/}
      {/*  <thead>*/}
      {/*    {*/}
      {/*      // Loop over the header rows*/}

      {/*      headerGroups.map((headerGroup: any) => (*/}
      {/*        // Apply the header row props*/}

      {/*        <tr {...headerGroup.getHeaderGroupProps()}>*/}
      {/*          {*/}
      {/*            // Loop over the headers in each row*/}

      {/*            headerGroup.headers.map((column: any) => (*/}
      {/*              // Apply the header cell props*/}

      {/*              <th {...column.getHeaderProps()}>*/}
      {/*                {*/}
      {/*                  // Render the header*/}

      {/*                  column.render("Header")*/}
      {/*                }*/}
      {/*              </th>*/}
      {/*            ))*/}
      {/*          }*/}
      {/*        </tr>*/}
      {/*      ))*/}
      {/*    }*/}
      {/*  </thead>*/}

      {/*  /!* Apply the table body props *!/*/}

      {/*  <tbody {...getTableBodyProps()}>*/}
      {/*    {*/}
      {/*      // Loop over the table rows*/}

      {/*      rows.map((row: any) => {*/}
      {/*        // Prepare the row for display*/}

      {/*        prepareRow(row);*/}

      {/*        return (*/}
      {/*          // Apply the row props*/}

      {/*          <tr {...row.getRowProps()}>*/}
      {/*            {*/}
      {/*              // Loop over the rows cells*/}

      {/*              row.cells.map((cell: any) => {*/}
      {/*                // Apply the cell props*/}

      {/*                return (*/}
      {/*                  <td {...cell.getCellProps()}>*/}
      {/*                    {*/}
      {/*                      // Render the cell contents*/}

      {/*                      cell.render("Cell")*/}
      {/*                    }*/}
      {/*                  </td>*/}
      {/*                );*/}
      {/*              })*/}
      {/*            }*/}
      {/*          </tr>*/}
      {/*        );*/}
      {/*      })*/}
      {/*    }*/}
      {/*  </tbody>*/}
      {/*</table>*/}
      <div>
        <button onClick={() => previousPage()} disabled={!canPreviousPage}>
          Previous Page
        </button>
        <button
          onClick={() => {
            loadNext(1);
          }}
          disabled={!canNextPage}
        >
          Next Page
        </button>
        <div>
          Page{" "}
          <em>
            {pageIndex + 1} of {pageOptions.length}
          </em>
        </div>
        <div>Go to page:</div>
        <input
          type="number"
          defaultValue={pageIndex + 1 || 1}
          onChange={(e) => {
            const page = e.target.value ? Number(e.target.value) - 1 : 0;
            gotoPage(page);
          }}
        />
        {/*<select*/}
        {/*  value={pageSize}*/}
        {/*  onChange={(e) => {*/}
        {/*    setPageSize(Number(e.target.value));*/}
        {/*  }}*/}
        {/*>*/}
        {/*  {pageSizeOptions.map((pageSize) => (*/}
        {/*    <option key={pageSize} value={pageSize}>*/}
        {/*      Show {pageSize}*/}
        {/*    </option>*/}
        {/*  ))}*/}
        {/*</select>*/}
      </div>
      <button
        onClick={() => {
          commit({
            variables: {},
            onError() {
              console.log("ter");
            },
            onCompleted() {
              console.log("compl");
            },
          });
        }}
      >
        login
      </button>
      <button
        onClick={() => {
          commit2({
            variables: {},
            onCompleted(resp) {
              console.log("cool", resp);
            },
            onError() {
              console.log("errr");
            },
          });
        }}
      >
        delete
      </button>
      {data.numbers.edges?.map((edge) => {
        return (
          <div key={edge?.node.id}>
            <Suspense fallback={"loadin"}>
              <h1>
                {edge?.node.id}
                {edge?.node.username}
              </h1>
            </Suspense>
          </div>
        );
      })}
      <button
        onClick={() => {
          loadPrevious(1);
        }}
      >
        loadprevious
      </button>
      <button
        onClick={() => {
          loadNext(1);
        }}
      >
        loadnext
      </button>
    </>
  );
}

function isEmpty(obj: any) {
  return Object.keys(obj).length === 0;
}

const pag = 3;
function App() {
  const [username, setusername] = useState<string | undefined>("");

  const [after, setafter] = useState<undefined | string>(undefined);
  const [first, setfirst] = useState<undefined | number>(pag);
  const [filter, setfilter] = useState<UsersFilterInput>({});

  useEffect(() => {
    console.log(username);
    if (username !== "") {
      // x ={...x, username:{like:username}}
      console.log("first", first, "after", after);
      setfilter({ ...filter, username: { like: username } });
    } else {
      if (filter?.hasOwnProperty("username")) {
        const { username, ..._filter } = filter;
        setfilter(_filter);
      }
      // delete filter?.username;
    }
    console.log(filter);
  }, [username]);

  const query = useLazyLoadQuery<AppQuery>(
    graphql`
      query AppQuery(
        $first: Int
        $after: String
        $before: String
        $last: Int
        $filter: UsersFilterInput
      ) {
        ...AppFragment
      }
    `,
    // { first: 2, after: "1" }
    {
      first: !isEmpty(filter) ? undefined : first,
      after: !isEmpty(filter) ? undefined : after,
      filter,
    }
  );

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <Upload />
        <Suspense fallback={"Loading"}>
          <Cool bla={query} />
        </Suspense>
        <form>
          <label>
            username
            <input
              type={"text"}
              name={"name"}
              value={username}
              onChange={(event) => {
                setusername(event.target.value);
              }}
            />
          </label>
        </form>
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
