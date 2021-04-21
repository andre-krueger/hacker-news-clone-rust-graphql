import React from 'react';
import Button from './Button';
import Box from './Box';
import Text from './Text';
import {View} from 'react-native';
import {graphql, useMutation} from 'react-relay';

function Delete() {
  const [commit2, bl] = useMutation(graphql`
    mutation LoginPageDeleteMutation {
      deleteUser(id: 10)
    }
  `);
  return (
    <Button
      text={'delete'}
      onPress={() =>
        commit2({
          variables: {},
          onCompleted(data) {
            console.log('bl', data);
          },
        })
      }
    />
  );
}

function LoginPage() {
  const [commit, _] = useMutation(graphql`
    mutation LoginPageMutation {
      login(username: "test", password: "test") {
        username
      }
    }
  `);
  return (
    <Box flex={1} backgroundColor={'primaryCardBackground'}>
      <Text>test</Text>
      <Button
        text={'login'}
        onPress={() =>
          commit({
            variables: {},
            onCompleted(data) {
              console.log('bl', data);
            },
          })
        }
      />
      <View style={{height: 100}} />
      <Delete />
      {/*<Button>Login</Button>;*/}
    </Box>
  );
}

export default LoginPage;
