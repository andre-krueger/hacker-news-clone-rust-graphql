import React, {
  MutableRefObject,
  Suspense,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
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
import { Link } from "react-router-dom";
import { Virtuoso, VirtuosoHandle } from "react-virtuoso";
import { ScrollerRef } from "react-virtuoso/dist/hooks/useScrollTop";

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
  const virtuoso: MutableRefObject<VirtuosoHandle | null> = useRef(null);
  useEffect(() => {
    if (
      virtuoso.current &&
      window.history.state
      // window.history.state.index
    ) {
      // console.log(virtuoso.current);
      // virtuoso.current.scrollToIndex(window.history.state.index);
      // virtuoso.current?.scrollTo({})
      // virtuoso.current?.scrollIntoView()
    }
  }, []);
  const { data, loadNext, loadPrevious } = usePaginationFragment(
    appFragment,
    bla
  );
  const [commit] = useMutation(feedbackLikeMutation);
  const [commit2] = useMutation(blafeedbackLikeMutation);

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
      <Virtuoso
        ref={virtuoso}
        data={data.numbers.edges!}
        initialTopMostItemIndex={window.history.state.index ?? 0}
        rangeChanged={(range) => {
          console.log("rangecha", range);
          const newstate = {
            ...window.history.state,
            index: range.startIndex,
          };
          window.history.pushState(newstate, "", null);
        }}
        // data={[{ node: { id: "1", username: "" } }]}
        style={{ height: "200px", width: "100%" }}
        // overscan={100}
        // onScroll={(event: React.UIEvent<"div", HTMLDivElement>) => {
        //   console.log("bla", event.currentTarget.scrollTop);
        // }}
        endReached={(index) => {
          loadNext(1);
        }}
        itemContent={(index, user) => {
          // return <h1>test</h1>;
          return (
            <Suspense fallback={"loading"}>
              <div key={user?.node.id}>
                <h1>
                  {user?.node.id}
                  {user?.node.username}
                </h1>
              </div>
            </Suspense>
          );
        }}
      />
      {/*{data.numbers.edges?.map((edge) => {*/}
      {/*  return (*/}
      {/*    <div key={edge?.node.id}>*/}
      {/*      <Suspense fallback={"loadin"}>*/}
      {/*        <h1>*/}
      {/*          {edge?.node.id}*/}
      {/*          {edge?.node.username}*/}
      {/*        </h1>*/}
      {/*      </Suspense>*/}
      {/*    </div>*/}
      {/*  );*/}
      {/*})}*/}
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

const pag = 1;
function App() {
  const [username, setusername] = useState<string | undefined>("");

  const [before, setbefore] = useState<undefined | string>(undefined);
  const [after, setafter] = useState<undefined | string>(undefined);
  const [last, setlast] = useState<undefined | number>(pag);
  const [first, setfirst] = useState<undefined | number>(undefined);
  const [filter, setfilter] = useState<UsersFilterInput>({});

  useEffect(() => {
    if (username !== "") {
      // x ={...x, username:{like:username}}
      setfilter({ ...filter, username: { like: username } });
    } else {
      if (filter?.hasOwnProperty("username")) {
        const { username, ..._filter } = filter;
        setfilter(_filter);
      }
      // delete filter?.username;
    }
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
      first: 2,
      // first: !isEmpty(filter) ? undefined : first,
      // last: !isEmpty(filter) ? undefined : last,
      // after: !isEmpty(filter) ? undefined : after,
      // before: !isEmpty(filter) ? undefined : before,
      // filter,
    },
    { fetchPolicy: "store-or-network" }
  );

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <Upload />
        <Link to={"/test"}>Bla</Link>
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
      </header>
    </div>
  );
}

export default App;
