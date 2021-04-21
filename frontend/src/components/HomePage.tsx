import React from 'react';
import Box from './Box';
import Text from './Text';
import {Button} from 'react-native';
import {useNavigation} from '@react-navigation/native';

function HomePage() {
  const {navigate} = useNavigation();
  return (
    <Box flex={1}>
      <Text>test</Text>
      <Button title={'Article 1'} onPress={() => navigate('ArticlePage')} />
    </Box>
  );
}

export default HomePage;
