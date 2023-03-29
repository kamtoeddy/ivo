import { Schema } from "./schema";

export default Schema;

export { isEqual } from "./utils/isEqual.js";
export { Schema } from "./schema";
export * as validate from "./validate";

// types & interfaces
export type {
  ContextType,
  ISchema,
  OperationSummary,
} from "./schema/interfaces";
export * from "./utils/interfaces";
