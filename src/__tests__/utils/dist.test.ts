import {
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize,
} from "../../../dist/utils/getUniqueBy";
import { belongsTo } from "../../../dist/utils/functions";
import { isEqual } from "../../../dist/utils/isEqual";

import { commonUtilTests } from ".";

commonUtilTests({
  belongsTo,
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize,
  isEqual,
});
