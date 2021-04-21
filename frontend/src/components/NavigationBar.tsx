import React from 'react';

import Icon from './Icon';
import {Pressable} from 'react-native';
import Box from './Box';
import {DrawerActions, useNavigation} from '@react-navigation/native';
import {useTheme} from '@shopify/restyle';
import {Theme} from '../theme';
import Button from './Button';

function NavigationBar() {
  const navigation = useNavigation();
  const theme = useTheme<Theme>();
  return (
    <Box
      flexBasis={'10%'}
      // alignItems={'center'}
      padding={'s'}
      style={{backgroundColor: 'red'}}>
      <Box flex={1} alignItems={'center'} flexDirection={'row'}>
        <Pressable
          android_ripple={{
            color: theme.colors.mainForeground,
            borderless: true,
          }}
          onPress={() => navigation.dispatch(DrawerActions.openDrawer)}>
          <Icon name={'menu'} />
        </Pressable>
      </Box>
    </Box>
  );
}

export default NavigationBar;
