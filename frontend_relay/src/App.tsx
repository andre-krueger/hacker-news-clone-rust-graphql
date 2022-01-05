import React, {
  createRef,
  MutableRefObject,
  Suspense,
  useCallback,
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
import ReactSlider from "react-slider";
import InfiniteLoader from "react-window-infinite-loader";
import { FixedSizeList, FixedSizeList as List } from "react-window";

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
      skip: $skip
    ) @connection(key: "AppFragment_numbers") {
      totalCount
      paginationVec
      edges {
        node {
          id
          username
        }
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }
`;

function Cool({
  skip,
  setskip,
  bla,
  setafter,
}: {
  skip: any;
  setskip: any;
  setafter: any;
  bla: AppFragment$key;
}) {
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
  const { data, loadNext, loadPrevious, refetch, isLoadingNext } =
    usePaginationFragment(appFragment, bla);
  const [isNextPageLoading, setIsNextPageLoading] = useState(false);
  const [hasNextPage, setHasNextPage] = useState(
    data.numbers.pageInfo.hasNextPage
  );
  const [timeline, settimeline] = useState(0);
  const [commit] = useMutation(feedbackLikeMutation);
  const [commit2] = useMutation(blafeedbackLikeMutation);
  const items = data.numbers.edges ?? [];
  const itemCount = data.numbers.pageInfo.hasNextPage
    ? items.length + 1
    : items.length;

  console.log(isLoadingNext);

  const loadMoreItems =
    // (false && !isLoadingNext) || isNextPageLoading
    isLoadingNext
      ? () => {}
      : (index: number) => {
          // setIsNextPageLoading(true);
          // setTimeout(() => {
          refetch({ after: data.numbers.pageInfo.endCursor });
          // loadNext(pag, {
          //   onComplete() {
          //     console.log("done");
          //     setHasNextPage(data.numbers.pageInfo.hasNextPage);
          //     setIsNextPageLoading(false);
          //   },
          // });
          // }, 1000);
          // setIsNextPageLoading(true);
        };

  const isItemLoaded = (index: number) =>
    !data.numbers.pageInfo.hasNextPage || index < items.length;

  const Item = ({ index, style }: { index: number; style: any }) => {
    let content;
    if (!isItemLoaded(index)) {
      content = "Loading...";
    } else {
      content = `${items[index]?.node.id}${items[index]?.node.username}`;
    }

    return <div style={style}>{content}</div>;
  };

  const myref: MutableRefObject<FixedSizeList<any> | null> = useRef(null);

  // const setRefs = useCallback(
  //   (node) => {
  //     if (typeof ref === "function") {
  //       ref(node);
  //     }
  //     ref.current = node;
  //   },
  //   [ref]
  // );

  useEffect(() => {
    if (myref.current) {
      console.log("yeah", window.history.state);
      if (window.history.state) {
        console.log("cool", window.history.state.scrollOffset);
        myref.current?.scrollTo(window.history.state.scrollOffset);
      }
    }
  }, [myref.current]);

  return (
    <div style={{ backgroundColor: "red", height: 50 }}>
      <InfiniteLoader
        isItemLoaded={isItemLoaded}
        loadMoreItems={loadMoreItems}
        itemCount={itemCount}
      >
        {({ onItemsRendered, ref: innerRef }) => (
          <List
            onScroll={({ scrollOffset }) => {
              if (scrollOffset > 0) {
                const newstate = {
                  ...window.history.state,
                  scrollOffset,
                };
                console.log("nnnn", scrollOffset);
                window.history.pushState(newstate, "", null);
              }
            }}
            // className="List"
            height={50}
            itemCount={itemCount}
            itemSize={30}
            onItemsRendered={onItemsRendered}
            // ref={(list) => {
            ref={(list) => {
              if (myref) {
                myref.current = list;
              }
              // @ts-ignore
              innerRef(list);
            }}
            //   if (ref) {
            //     ref.current = list;
            //   }
            //   // if (innerRef) {
            //   //   innerRef(list);
            //   // }
            // }}
            width={300}
          >
            {Item}
          </List>
        )}
      </InfiniteLoader>
    </div>
  );

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
      <div style={{ position: "absolute", right: 30, height: 100, width: 100 }}>
        <ReactSlider
          className="vertical-slider"
          // thumbClassName="example-thumb"
          // trackClassName="example-track"
          // defaultValue={10}
          min={1}
          value={timeline}
          max={data.numbers.paginationVec.length}
          // defaultValue={[0, 50, 100]}
          // ariaLabel={["Lowest thumb", "Middle thumb", "Top thumb"]}
          renderThumb={(props, state) => (
            <div {...props}>
              {state.valueNow}
              {data.numbers.paginationVec[state.valueNow - 1]}
              {/*{data.numbers.edges?[state.valueNow]?.node.username??""}*/}
            </div>
          )}
          orientation="vertical"
          invert
          pearling={false}
          onBeforeChange={() => {
            console.log("bla");
          }}
          onAfterChange={(n) => {
            if (virtuoso.current) {
              // setafter(data.numbers.paginationVec[n]);
              // loadNext(10);
              // refetch({
              //   first: 1,
              //   after: data.numbers.pageInfo.endCursor,
              //   skip: Math.max(n - 1, 0),
              //
              //   // after: data.numbers.paginationVec[Math.max(0, n - 1)],
              // });

              const c = data.numbers.edges?.length;
              const cc = data.numbers.paginationVec.length;
              console.log(c, cc);
              refetch({
                skip: skip + n,
                after: data.numbers.paginationVec[5],
              });
              // if (c === cc) {
              //   virtuoso.current?.scrollToIndex({
              //     index: n - 1,
              //     // align: "start",
              //     // behavior: "auto",
              //   });
              // } else {
              //   loadNext(n, {
              //     onComplete() {
              //       virtuoso.current?.scrollToIndex({
              //         index: n - 1,
              //         // align: "start",
              //         // behavior: "auto",
              //       });
              //     },
              //   });
              // }
            }
          }}
          // minDistance={10}
        />
      </div>
      <Virtuoso
        ref={virtuoso}
        // initialItemCount={data.numbers.paginationVec.length}
        // initialItemCount={10}
        data={data.numbers.edges!}
        initialTopMostItemIndex={window.history.state.index ?? 0}
        rangeChanged={(range) => {
          settimeline(range.startIndex + 1);
          console.log("test", range);
          const newstate = {
            ...window.history.state,
            index: range.startIndex,
          };
          window.history.pushState(newstate, "", null);
        }}
        // data={[{ node: { id: "1", username: "" } }]}
        style={{ height: "100px", width: "100%" }}
        // overscan={100}
        // onScroll={(event: React.UIEvent<"div", HTMLDivElement>) => {
        //   console.log("bla", event.currentTarget.scrollTop);
        // }}
        // atTopStateChange={(b) => {
        //   if (b) {
        //     console.log("start");
        //     // loadPrevious(1);
        //     refetch({
        //       skip: 0,
        //       first: undefined,
        //       after: undefined,
        //       before: "2023-10-10",
        //       last: 3,
        //     });
        //     // loadPrevious(1);
        //   }
        // }}
        // endReached={(index) => {
        //   // console.log("cool endreached", index);
        //   // loadNext(1);
        //   // if (index !== 0) {
        //   refetch({
        //     skip: index,
        //     first: 3,
        //     before: undefined,
        //     last: undefined,
        //     after: data.numbers.pageInfo.endCursor,
        //   });
        //   // refetch({
        //   //   first: 3,
        //   //   skip: index > 1 ? 10 : undefined,
        //   //   // after: data.numbers.pageInfo.endCursor,
        //   // });
        //   // }
        //   loadNext(1);
        // }}
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
  const [skip, setskip] = useState<number | undefined>(undefined);

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
        $skip: Int
      ) {
        ...AppFragment
      }
    `,
    // { first: 2, after: "1" }
    {
      // first: pag,
      // skip,
      first: !isEmpty(filter) ? undefined : pag,
      // last: !isEmpty(filter) ? undefined : last,
      after: !isEmpty(filter) ? undefined : after,
      // before: !isEmpty(filter) ? undefined : before,
      filter,
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
          <Cool setafter={setafter} bla={query} skip={skip} setskip={setskip} />
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
