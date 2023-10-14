import {
  getUnique,
  getUniqueBy,
  isOneOf,
  isEqual,
  isNullOrUndefined
} from '../../src/utils';

import { commonUtilTests } from '.';

commonUtilTests({
  isOneOf,
  getUnique,
  getUniqueBy,
  isEqual,
  isNullOrUndefined
});
