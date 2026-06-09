import {
  getUnique,
  getUniqueBy,
  isEqual,
  isNullOrUndefined,
  isOneOf,
} from '../../src/utils';

import { commonUtilTests } from '.';

commonUtilTests({
  isOneOf,
  getUnique,
  getUniqueBy,
  isEqual,
  isNullOrUndefined,
});
