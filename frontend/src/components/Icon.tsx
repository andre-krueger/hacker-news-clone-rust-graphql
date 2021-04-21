import React from 'react';
import VectorIcon from 'react-native-vector-icons/MaterialIcons';
import {useTheme} from '@shopify/restyle';
import {Theme} from '../theme';

function Icon({name}: {name: string}) {
  const theme = useTheme<Theme>();
  return <VectorIcon name={name} style={{...theme.textVariants.icon}} />;
}

export default Icon;
