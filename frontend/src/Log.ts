import {logger} from 'react-native-logs';

const Log = logger.createLogger({
  enabled: __DEV__,
});

export default Log;
