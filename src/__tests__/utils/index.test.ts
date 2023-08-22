import {
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize
} from '../../utils/getUniqueBy'
import { belongsTo, isEqual } from '../../utils/functions'

import { commonUtilTests } from '.'

commonUtilTests({
  belongsTo,
  getDeepValue,
  getUnique,
  getUniqueBy,
  serialize,
  isEqual
})
