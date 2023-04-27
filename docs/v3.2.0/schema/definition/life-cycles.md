# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

```ts
import type { Context } from "clean-schema";

type Input = {};

type Output = {};

type IContext = Context<Input, Output>;

const Model = new Schema<Input, Output>(definitions).getModel();

type DeleteListener = (data: Readonly<Output>) => void | Promise<void>;

type Listener = (context: IContext) => void | Promise<void>;
```

# The Operation Summary

```ts
import type { Context, Summary } from "clean-schema";

type Input = {};

type Output = {};

type IContext = Context<Input, Output>;
type ISummary = Summary<Input, Output>;

// 👇 this is what `Summary` looks like
type S =
  | Readonly<{
      context: IContext;
      operation: "creation";
      previousValues: undefined;
      values: Readonly<Output>;
    }>
  | Readonly<{
      context: IContext;
      operation: "update";
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>;

const Model = new Schema<Input, Output>(definitions).getModel();

type Handler = (context: IContext) => void | Promise<void>;

type SuccessHandler = (summary: ISummary) => void | Promise<void>;
```

## Life Cycle handlers

These are functions that are invoked during a life cycle operation (`creation`, `failure` or `update`)

### onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. These listeners have access to a context without sideEffects even if passed to the delete method of the model. Default **[ ]**. They are expected to respect the `type Handler` as shown above

### onFailure

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are unsuccessful. Default **[ ]**. They are expected to respect the `type SuccessHandler` as shown above

> N.B: They are only allowed on properties that support and have validators

### onSuccess

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are successful. Handlers for this event should expect the operation's context object & the concerned operation name (`creating` | `updating`) as first & second parameters respectively. Default **[ ]**.They are expected to respect the `type SuccessHandler` as shown above

As from `v2.5.0`, these handlers have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create, clone & update methods of your models.

If the operation is unsuccessful, `handleSuccess` will be `undefined`

```js
const { data, error, handleSuccess } = await UserModel.create(userData);

if (data) await handleSuccess();
```
