import React from "react";
import VectorIcon from "react-native-vector-icons/MaterialIcons";

type Size = 16 | 32;

function Icon({ name, size }: { name: string; size: Size }): JSX.Element {
  return <VectorIcon name={name} size={size} />;
}

export default Icon;
