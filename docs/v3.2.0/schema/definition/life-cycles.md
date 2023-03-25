# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

```ts
import type { CombinedType } from "clean-schema";

type Input = {};

type Output = {};

const Model = new Schema<Input, Output>(definitions).getModel();

type DeleteListener = (context: Output) => void | Promise<void>;

type Listener = (context: CombinedType<Input, Output>) => void | Promise<void>;

// on property success
type OperationSummary<Property> =
  | {
      context: CombinedType<Input, Output>;
      operationName: "creation";
      previousValue: undefined;
      value: CombinedType<Input, Output>[Property];
    }
  | {
      context: CombinedType<Input, Output>;
      operationName: "update";
      previousValue: CombinedType<Input, Output>[Property];
      value: CombinedType<Input, Output>[Property];
    };

type SuccessListener = (
  operationSummary: CombinedType<Input, Output>
) => void | Promise<void>;
```

## Life Cycle listeners

These are functions that are invoked during a life cycle operation and recieve the [operation context](#the-operation-context) as only parameter. They are expected to respect the `type Listener` as shown above

## onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. These listeners have access to a context without sideEffects even if passed to the delete method of the model. Default **[ ]**. They are expected to respect the `type DeleteListener` as shown above

## onFailure

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are unsuccessful. Default **[ ]**. They are expected to respect the `type Listener` as shown above

> N.B: They are only allowed on properties that support and have validators

## onSuccess

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are successful. Listeners for this event should expect the operation's context object & the concerned operation name (`creating` | `updating`) as first & second parameters respectively. Default **[ ]**.They are expected to respect the `type SuccessListener` as shown above

As from `v2.5.0`, these listeners have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create, clone & update methods of your models.

If the operation is unsuccessful, `handleSuccess` will be `undefined`

```js
const { data, error, handleSuccess } = await UserModel.create(userData);

await handleSuccess?.();
```
