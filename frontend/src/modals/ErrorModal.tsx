import React from "react";
import { Button, Modal, Text } from "react-native";

type Props = {
  visible: boolean;
  resetError: () => void;
};

function ErrorModal({ visible, error, resetError }: Props): JSX.Element {
  console.log("error", error);
  return (
    <Modal visible={visible}>
      <Text>test</Text>
      <Button title={"retry"} onPress={() => resetError()} />
    </Modal>
  );
}

export default ErrorModal;
