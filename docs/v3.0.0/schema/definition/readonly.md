# Readonly Properties

Such a property's value will be changed at most twice depending on ypur usecase. Any attempt to modify the value after it has changed will be ignored.

If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false**

Just like [dependent properties](./dependents.md#dependent-properties) they `must` have a default value and can be used in combination with other rules like [**dependent**](./dependents.md#dependent-properties), **shouldInit**, etc.

They **`cannot be strictly required`** but can be [conditionally required](./required.md#conditionally-required-properties)

> Out of the box, dependent is assumed to be **`false`** for every property

Example:

```ts
import { Schema } from "clean-schema";

const orderSchema = new Schema({
  completedAt: {
    default: "",
    readonly: true,
    dependent: true,
    dependsOn: "isComplete",
    resolver: ({ isComplete }) => (isComplete ? new Date() : ""),
  },
  isComplete: {
    default: false,
    readonly: true,
    shouldInit: false,
    validator: validateBoolean,
  },
  receiptNumber: {
    default: null,
    readonly: "lax",
    validator: validateReceipt,
  },
});
```
