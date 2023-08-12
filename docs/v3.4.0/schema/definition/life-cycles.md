# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

# The Operation Summary

```ts
import type { Context, Summary } from 'clean-schema'

type Input = {}
type Output = {}

type IContext = Context<Output, Input>
type ISummary = Summary<Output, Input>

// 👇 S below is the same as `ISummary`
// 👇 this is what `ISummary` looks like
type S =
  | Readonly<{
      changes: null
      context: IContext
      operation: 'creation'
      previousValues: null
      values: Readonly<Output>
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>
      context: IContext
      operation: 'update'
      previousValues: Readonly<Output>
      values: Readonly<Output>
    }>

const Model = new Schema<Output, Input>(definitions).getModel()

type Handler = (context: IContext) => void | Promise<void>

type HandlerWithSummary = (summary: ISummary) => void | Promise<void>
```

## Life Cycle handlers

These are functions that are invoked during a life cycle operation (`creation`, `failure` or `update`)

### onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. These listeners have access to a context without sideEffects even if passed to the delete method of the model. Default **[ ]**. They are expected to respect the `type Handler` as shown above

### onFailure

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are unsuccessful. Default **[ ]**. They are expected to respect the `type HandlerWithSummary` as shown above

> N.B: They are only allowed on properties that support and have validators

### onSuccess

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are successful. Handlers for this event should expect the operation's summary as only parameter. Default **[ ]**. They are expected to respect the `type HandlerWithSummary` as shown above

As from `v2.5.0`, these handlers have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create, clone & update methods of your models.

If the operation is unsuccessful, `data` and `handleSuccess` will be `null`

```js
const { data, error, handleSuccess } = await UserModel.create(userData)

if (data) await handleSuccess()
```
