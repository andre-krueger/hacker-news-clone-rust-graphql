// import React from "react";
// import { Button, View } from "react-native";
// import { graphql, useMutation } from "react-relay";
// import { bla, getItem, setItem } from "./Storage";
//
// async function ctest() {}
// function Delete() {
//   const [commit2, bl] = useMutation(graphql`
//     mutation LoginScreenDeleteMutation {
//       deleteUser(id: 10)
//     }
//   `);
//   // ctest();
//   // try {
//   // setItem('cookie', '').mapErr(e => {
//   //   console.log(e);
//   // }).;
//   // setItem('cookie', '')
//   //   .map(() => {})
//   //   .andThen(() => {});
//   setItem("cookie", "cool").mapErr((e) => {
//     console.log(e);
//   });
//   // getItem('cookie').then(res => {
//   //   console.log(res);
//   //   if (res.isOk()) {
//   //     res.un;
//   //   }
//   // });
//   getItem("cookie").map((e) => {
//     console.log("test", e);
//   });
//   // let nn = getItem('cookie').match<any>(
//   //   item => {
//   //     console.log('enrtntendxend', item);
//   //     return Some(item);
//   //   },
//   //   error => {},
//   // );
//   // console.log('enxnedxnebbbbbbbbbbbbbbbbbbbbbb', nn);
//   // const t = getItem('cookie').match(
//   //   item => {
//   //     return Some(item);
//   //     // returconsole.log('cool', item);
//   //   },
//   //   error => {
//   //     console.log('nooo', error);
//   //   },
//   // );
//   // console.log(t.then(b => b.unwrap()));
//   // // }
//   // // catch (e) {}
//   // // setItem().then(t => {
//   // //   t.ok();
//   // // });
//   // .setItem('cookie', '')
//   // .then(e =>
//   //   e.match({
//   //     ok: _ => 0,
//   //     err: _ => 1,
//   //   }),
//   // );
//   return (
//     <Button
//       title={"delete"}
//       onPress={() =>
//         commit2({
//           variables: {},
//           onCompleted(data) {
//             console.log("bl", data);
//           },
//         })
//       }
//     />
//   );
// }
//
// function LoginScreen() {
//   const [commit, _] = useMutation(graphql`
//     mutation LoginScreenMutation {
//       login(username: "test", password: "test") {
//         username
//       }
//     }
//   `);
//   return (
//     <View>
//       <Button
//         title={"login"}
//         onPress={() =>
//           commit({
//             variables: {},
//             onCompleted(data) {
//               console.log("bl", data);
//             },
//           })
//         }
//       />
//       <Delete />
//     </View>
//   );
// }
//
// export default LoginScreen;
