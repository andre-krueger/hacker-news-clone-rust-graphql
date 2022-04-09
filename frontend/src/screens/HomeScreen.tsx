import React from "react";
import { useTranslation } from "react-i18next";
import Box from "../components/Box";
import Text from "../components/Text";
import { StackScreenProps } from "@react-navigation/stack";
import { HomeStackParamList } from "../navigation/HomeStack";
import Button from "../components/Button";

type Props = StackScreenProps<HomeStackParamList, "HomeScreen">;

function HomeScreen({ navigation: { navigate } }: Props): JSX.Element {
  const { t } = useTranslation();
  return (
    <Box flex={1}>
      <Text>test</Text>
      <Button title={t("hello")} onPress={() => navigate("ArticleScreen")} />
      {/*<Button title={t("hello")} onPress={() => navigate("ArticleScreen")} />*/}
    </Box>
  );
}

export default HomeScreen;
