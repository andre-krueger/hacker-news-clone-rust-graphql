import React from "react";
import Box from "../components/Box";
import { StackScreenProps } from "@react-navigation/stack";
import { HomeStackParamList } from "../navigation/HomeStack";

type Props = StackScreenProps<HomeStackParamList, "ArticleScreen">;

function ArticleScreen(): JSX.Element {
  return <Box />;
}

export default ArticleScreen;
