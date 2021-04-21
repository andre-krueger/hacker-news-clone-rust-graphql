import React from 'react';
import Box from './Box';
import Text from './Text';
import {Pressable} from 'react-native';
import {useTheme} from '@shopify/restyle';
import {Theme} from '../theme';

function Button({
  text,
  numberOfLines = 1,
  onPress,
}: {
  text: string;
  numberOfLines?: number;
  onPress: () => void;
}) {
  const theme = useTheme<Theme>();
  return (
    <Pressable
      android_ripple={{
        color: theme.colors.mainForeground,
        borderless: true,
      }}
      onPress={() => onPress()}>
      <Box
        flexShrink={1}
        backgroundColor={'mainBackground'}
        style={{backgroundColor: 'red'}}>
        <Text
          adjustsFontSizeToFit
          numberOfLines={numberOfLines}
          color={'mainForeground'}>
          {text}
        </Text>
      </Box>
    </Pressable>
  );
}

export default Button;
