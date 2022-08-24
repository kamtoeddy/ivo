import {
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize,
} from "../../../lib/utils/getUniqueBy";
import { belongsTo } from "../../../lib/utils/functions";
import { isEqual } from "../../../lib/utils/isEqual";

import { commonUtilTests } from "../common-tests/utils";

commonUtilTests({
  belongsTo,
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize,
  isEqual,
});
