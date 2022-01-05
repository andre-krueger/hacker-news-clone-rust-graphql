import React, {
  Dispatch,
  MutableRefObject,
  SetStateAction,
  useEffect,
  useRef,
  useState,
} from "react";
import logo from "./logo.svg";
import "./App.css";
import {
  OrderBy,
  TestQuery,
  UsersFilterInput,
  useTestQuery,
} from "./generated/graphql";
import { UseQueryState } from "urql";
import ReactPaginate from "react-paginate";

import { FixedSizeList, FixedSizeList as List } from "react-window";

import ReactSlider from "react-slider";
import { Virtuoso, VirtuosoHandle } from "react-virtuoso";
import InfiniteLoader from "react-window-infinite-loader";
import { Link } from "react-router-dom";

let pag = 2;

function Bla({
  count,
  setcount,
  numbers,
  setAfter,
  username,
  handleChange,
  page,
  setpage,
  setBefore,
  setfirst,
  setlast,
  setskip,
  skip,
  after,
  setback,
  pagesize,
  setpagesize,
}: {
  setAfter: Dispatch<SetStateAction<string | null | undefined>>;
  numbers: UseQueryState<TestQuery, object>;
  handleChange: any;
  setcount: any;
  count: any;
  username: any;
  page: any;
  setskip: any;
  setpage: any;
  setBefore: any;
  setfirst: any;
  setpagesize: any;
  pagesize: any;
  setlast: any;
  skip: any;
  after: any;
  setback: any;
}) {
  let _count = numbers.data?.numbers.totalCount ?? 0;
  const first = useRef(true);
  useEffect(() => {
    // if (first.current && _count > 0) {
    // setcount(count - 1 + _count);
    if (count !== _count) {
      setcount(Math.min(_count, count + Math.abs(count - _count)));
    }
    // first.current = false;
    // }
  }, [_count]);
  const firstrender = useRef(true);
  if (!numbers) {
    return <h1>test</h1>;
  }
  const startCursor = numbers.data?.numbers.pageInfo.startCursor;
  const endCursor = numbers.data?.numbers.pageInfo.endCursor;
  return (
    <>
      {numbers.data?.numbers.edges?.map((edge) => {
        return (
          <div key={edge?.node.id}>
            <h1>
              {edge?.node.id}
              {edge?.node.username}
            </h1>
          </div>
        );
      })}
      <ReactPaginate
        pageCount={Math.ceil(count / pag)}
        pageRangeDisplayed={1}
        onPageChange={({ selected }) => {
          if (page > selected) {
            // setcount(count - _count);
            setback(true);
            setAfter(startCursor);
          } else {
            // setcount(count + _count);
            setback(false);
            setAfter(endCursor);
          }
          const pagdiff = Math.abs(selected - page);
          if (pagdiff > 1) {
            setskip((pagdiff - 1) * pag);
          } else {
            setskip(0);
          }
          setpage(selected);
        }}
      />
      {/*<ReactPaginate*/}
      {/*  pageCount={Math.ceil(count / pag)}*/}
      {/*  onPageChange={({ selected }) => {*/}
      {/*    // const doesSkip = selected - 1 - page > 0;*/}
      {/*    setpage(selected);*/}
      {/*    // console.log("page", page, "test", selected - 1, "doeskip", doesSkip);*/}
      {/*    // if (doesSkip) {*/}
      {/*    //   const _skip = Math.abs(page - selected + 1) * pag;*/}
      {/*    //   setskip(_skip);*/}
      {/*    // } else {*/}
      {/*    //   setskip(0);*/}
      {/*    // }*/}
      {/*    console.log("after", after);*/}
      {/*    if (page < selected) {*/}
      {/*      setAfter(endCursor);*/}
      {/*    } else {*/}
      {/*      setAfter(`-${startCursor}`);*/}
      {/*    }*/}
      {/*    // if (page < selected) {*/}
      {/*    //   setAfter(endCursor);*/}
      {/*    // } else {*/}
      {/*    //   const _doesSkip = page - selected - 1 > 0;*/}
      {/*    //   const _skip = Math.abs(selected - page + 2) * pag;*/}
      {/*    //   if (_doesSkip) {*/}
      {/*    //     setAfter(`-${parseInt(endCursor!, 10)}`);*/}
      {/*    //     setskip(_skip);*/}
      {/*    //     // setskip(0);*/}
      {/*    //     console.log("ohno", (selected - page + 2) * pag);*/}
      {/*    //   } else {*/}
      {/*    //     setskip(0);*/}
      {/*    //     console.log("coontuentdn");*/}
      {/*    //     setAfter(`-${parseInt(endCursor!, 10)}`);*/}
      {/*    //     // const _skip = Math.abs(page - selected + 1) * pag;*/}
      {/*    //     // setskip(_skip);*/}
      {/*    //   }*/}
      {/*    // }*/}
      {/*    // console.log("offset", page - selected);*/}
      {/*    // // console.log("cool", selected * pag);*/}
      {/*    // // const b = Math.ceil(parseInt("3", 10) / selected);*/}
      {/*    // // let t = Math.min(total-1,selected * parseInt(after ?? "0",10))*/}
      {/*    // // console.log('befort',after,t)*/}
      {/*    // // t=5*/}
      {/*    // // if (t === pag) {*/}
      {/*    // //   t++;*/}
      {/*    // // }*/}
      {/*    // // setAfter(`${after}`);*/}
      {/*    // // if (page <= selected) {*/}
      {/*    // //   setfirst(pag);*/}
      {/*    // //   setBefore(undefined);*/}
      {/*    // //   setlast(undefined);*/}
      {/*    // //   setAfter(endCursor);*/}
      {/*    // // } else {*/}
      {/*    // //   setAfter(undefined);*/}
      {/*    // //   setfirst(undefined);*/}
      {/*    // //   setlast(pag);*/}
      {/*    // //   setBefore(startCursor);*/}
      {/*    // // }*/}
      {/*    // // console.log(selected * pag);*/}
      {/*    //*/}
      {/*    // if (doesSkip) {*/}
      {/*    //   setskip(Math.abs(page - selected) * pag);*/}
      {/*    //   // setskip(selected * pag);*/}
      {/*    // } else {*/}
      {/*    //   // console.log("noo");*/}
      {/*    //   setskip(0);*/}
      {/*    // }*/}
      {/*    // if (page <= selected) {*/}
      {/*    //   // setfirst(pag);*/}
      {/*    //   // setBefore(undefined);*/}
      {/*    //   // setlast(undefined);*/}
      {/*    //   // setAfter(endCursor);*/}
      {/*    //   if (!doesSkip) {*/}
      {/*    //     setAfter(endCursor);*/}
      {/*    //   }*/}
      {/*    // } else {*/}
      {/*    //   // console.log("star", startCursor);*/}
      {/*    //   // setfirst(undefined);*/}
      {/*    //   // setlast(pag);*/}
      {/*    //   // setAfter(undefined);*/}
      {/*    //   // setBefore(startCursor);*/}
      {/*    //   // setAfter(`${parseInt(startCursor!) - pag * pag}`);*/}
      {/*    //   // console.log("nnn", Math.abs(page - selected) * pag);*/}
      {/*    //   // if (skip > 0) {*/}
      {/*    //   //   console.log("itis");*/}
      {/*    //   //   // setAfter(undefined);*/}
      {/*    //   //   // setskip(0);*/}
      {/*    //   //   console.log((page - selected) * pag);*/}
      {/*    //   //   setskip(Math.abs(page - selected) * pag);*/}
      {/*    //   //   setAfter("-2");*/}
      {/*    //   //   // setAfter(*/}
      {/*    //   //   //   `${*/}
      {/*    //   //   //     -(parseInt(startCursor!, 10) - Math.abs(page - selected)) **/}
      {/*    //   //   //     pag*/}
      {/*    //   //   //   }`*/}
      {/*    //   //   // );*/}
      {/*    //   // }*/}
      {/*    //   // else {*/}
      {/*    //   // console.log("itis");*/}
      {/*    //   setskip(Math.abs(selected) * pag);*/}
      {/*    //   setAfter(`-${parseInt(startCursor!, 10)}`);*/}
      {/*    //   // }*/}
      {/*    // }*/}
      {/*    // console.log("ubernn", (selected - page) * pag);*/}
      {/*    // console.log(page, selected);*/}
      {/*    // console.log("does", doesSkip);*/}
      {/*    // setskip((selected - page) * pag);*/}
      {/*    // console.log("ozy", selected, page);*/}
      {/*    // setAfter(endCursor);*/}
      {/*    // setAfter(endCursor);*/}
      {/*    // setAfter(`${Math.min(selected * pag, parseInt(endCursor!, 10))}`);*/}
      {/*    // setAfter(`${count / pag}`);*/}
      {/*    // setAfter(`${(selected -1) * count -1}`)*/}
      {/*  }}*/}
      {/*  nextLabel={null}*/}
      {/*  previousLabel={null}*/}
      {/*/>*/}
      {/*<Pagination*/}
      {/*  items={Math.ceil(numbers.data?.numbers.totalCount! / pag)}*/}
      {/*  onClick={(sel) => {*/}
      {/*    console.log(sel);*/}
      {/*    setAfter(`${sel}`);*/}
      {/*  }}*/}
      {/*/>*/}

      <form>
        <label>
          username
          <input
            type={"text"}
            name={"name"}
            value={username}
            onChange={handleChange}
          />
        </label>
      </form>
    </>
  );
  // // if (numbers.data?.numbers.__typename) {
  // switch (numbers.data?.numbers.__typename) {
  //   case "PaginationIncorrect": {
  //     return <h1>Incorrect pagination</h1>;
  //   }
  //   case "UserConnection": {
  //     let arr = range(1, numbers.data?.numbers.totalCount / pag + 1);
  //     console.log(arr);
  //     let b = [1, 2, 3];
  //     console.log(numbers.data?.numbers);
  //     let total=numbers.data?.numbers.totalCount;
  //     let after = "0"//numbers.data?.numbers.pageInfo.startCursor ?? "5";
  //     // if (numbers.data?.numbers.pageInfo.hasNextPage){
  //     //   after=numbers.data.numbers.pageInfo.endCursor as string
  //     // }
  //     // if (numbers.data?.numbers.pageInfo.hasPreviousPage){
  //     //
  //     //   after=numbers.data.numbers.pageInfo.startCursor as string
  //     // }
  //     console.log('oz,',after);
  //     return (
  //       <>
  //         <h1>{numbers.data?.numbers.totalCount}</h1>
  //         {numbers.data?.numbers.edges?.map((b) => (
  //           <div key={b?.node.id}>
  //             <h3>{b?.node.username}</h3>
  //             <h3>{b?.node.id}</h3>
  //           </div>
  //         ))}
  //         {/*<Pagination*/}
  //         {/*  items={Math.ceil(numbers.data?.numbers.totalCount / pag)}*/}
  //         {/*  onClick={() => {}}*/}
  //         {/*/>*/}
  //         {/*{numbers.data?.numbers.pageInfo.hasNextPage && <button>Click</button>}*/}
  //         <ReactPaginate
  //           pageCount={Math.ceil(numbers.data?.numbers.totalCount / pag)}
  //           onPageChange={({ selected }) => {
  //             // console.log("cool", selected * pag);
  //             // const b = Math.ceil(parseInt("3", 10) / selected);
  //             // let t = Math.min(total-1,selected * parseInt(after ?? "0",10))
  //             // console.log('befort',after,t)
  //             // t=5
  //             // if (t === pag) {
  //             //   t++;
  //             // }
  //             setAfter(`${after}`);
  //           }}
  //         />
  //       </>
  //     );
  //     // return numbers.numbers;
  //     // return <h1>test</h1>;
  //   }
  //   // return (
  //   //   <>
  //   //     {(numbers.data?.numbers as any).edges?.map((b: any) => (
  //   //       <div key={b?.node.id}>
  //   //         <h3>{b?.node.username}</h3>
  //   //         <h3>{b?.node.id}</h3>
  //   //       </div>
  //   //     ))}
  //   //   </>
  //   // );
  // }
  // // } else {
  // //   // let x = numbers.data?.numbers as TestQuery;
  // //   // x.numbers
  // // }
  // return null;
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

function isEmpty(obj: any) {
  return Object.keys(obj).length === 0;
}

function Cool({
  setAfter,
  before,
  setBefore,
  after,
}: {
  before: any;
  setBefore: any;
  after: string | null | undefined;
  setAfter: Dispatch<SetStateAction<string | null | undefined>>;
}) {
  const [first, setfirst] = useState(pag);

  const [skip, setskip] = useState<number | undefined>(undefined);
  const [username, setusername] = useState("");
  const [filter, setfilter] = useState<UsersFilterInput>({});
  useEffect(() => {
    if (username !== "") {
      setfilter({ ...filter, username: { like: username } });
    } else {
      const { username, ..._filter } = filter;
      setfilter(_filter);
    }
  }, [username]);

  const virtuoso: MutableRefObject<VirtuosoHandle | null> = useRef(null);
  console.log("skip", skip);
  const [res, exec] = useTestQuery({
    variables: {
      first: !isEmpty(filter) ? undefined : pag,
      after: !isEmpty(filter) ? undefined : after,
      filter,
    },
  });

  const items = res.data?.numbers.edges ?? [];
  const itemCount = res.data?.numbers.pageInfo.hasNextPage
    ? items.length + 1
    : items.length;

  const loadMoreItems =
    // (false && !isLoadingNext) || isNextPageLoading
    res.fetching
      ? () => {}
      : (index: number) => {
          // setIsNextPageLoading(true);
          // setTimeout(() => {
          // refetch({ after: data.numbers.pageInfo.endCursor });
          // exec({ after: res.data?.numbers.pageInfo.endCursor });
          setAfter(res.data?.numbers.pageInfo.endCursor);
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
    !res.data?.numbers.pageInfo.hasNextPage || index < items.length;

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

  useEffect(() => {
    if (myref.current) {
      console.log("yeah", window.history.state);
      if (window.history.state) {
        console.log("cool", window.history.state.scrollOffset);
        myref.current?.scrollTo(window.history.state.scrollOffset);
      }
    }
  }, [myref.current]);

  console.log("isfe", res.fetching);

  return (
    <>
      <Link to={"/test"}>Bla</Link>
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
    </>
  );
}

function B() {
  return <h1></h1>;
}

function App() {
  // const [res, exec] = useTestQuery({ variables: { first: 3 } });
  // res.data?.numbers.__typename==='PaginationIncorrect';
  // res.data;
  // console.log(res);
  const [after, setAfter] = useState<string | null | undefined>(undefined);
  const [before, setBefore] = useState<string | null | undefined>(undefined);
  // const [res, ex] = useLoginMutation();
  // const [res_, exec] = useDelMutation();
  // console.log("cool", ex());
  // useEffect(() => {
  //   console.log("aber", res_);
  // }, [res_]);

  const [pageVariables, setPageVariables] = useState([
    {
      first: 10,
      after: null,
    },
  ]);

  return (
    <div className="App">
      <header className="App-header">
        {/*<button*/}
        {/*  onClick={() => {*/}
        {/*    ex({ username: "admin", password: "admin" });*/}
        {/*  }}*/}
        {/*>*/}
        {/*  login*/}
        {/*</button>*/}
        {/*<button*/}
        {/*  onClick={() => {*/}
        {/*    exec();*/}
        {/*  }}*/}
        {/*>*/}
        {/*  Del*/}
        {/*</button>*/}
        {/*<button onClick={() => exec( )}>Refetch</button>*/}
        <Cool
          setAfter={setAfter}
          after={after}
          setBefore={setBefore}
          before={before}
        />
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
