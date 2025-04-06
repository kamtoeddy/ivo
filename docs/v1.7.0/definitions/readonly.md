# Readonly Properties

Such a property's value will be changed at most twice depending on your usecase. Any attempt to modify the value after it has changed will be ignored.

- If set to `true`, will be required at initialization and will never allow updates.
- If set to `true` with shouldInit: false, will not be initialized but allowed to update only once.
- If set to `lax`, will not be required at creation nor during updates (unless conditionally required). Once it's value is different from the default value, it'll not accept further updates.

They **`cannot be strictly required`** but can be [conditionally required](./required.md#conditionally-required-properties)

They should have a default value if they are [dependent](./dependents.md#dependent-properties), [conditionally required](./required.md#conditionally-required-properties) or have their initialization blocked (i.e. `shouldInit: false`)

Example:

```ts
import { Schema } from "ivo";

const orderSchema = new Schema({
  completedAt: {
    default: "",
    readonly: true,
    dependsOn: "isComplete",
    resolver: ({ context: { isComplete } }) => (isComplete ? new Date() : ""),
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
