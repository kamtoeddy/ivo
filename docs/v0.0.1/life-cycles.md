# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

This object also has a method `__getOptions__` that return a readonly copy of the context options provided at creation, updates and deletions. [more here](#context-options)

```ts
import type { Context } from 'ivo';

type Input = { virtualProp: string };
type Output = { constantProp: string; dependentProp: string };
type CtxOptions = { lang: 'en' };

type CTX = Context<Input, Output, CtxOptions>;

// 👇 this is what `CTX` above would look like
type C = {
  __getOptions__: () => Readonly<CtxOptions>;
  constantProp: string;
  dependentProp: string;
  virtualProp: string;
};
```

## Context Options

This is a way of providing extra information (releted or not related to you schema) to operations like creation, updates and deletion.A good usecase would be internationalization (i18n)

How to use:

```ts
// 1) pass it to related operations

// creating an entity   👇
Model.create(input, { lang: 'en' });

// updating an entity             👇
Model.update(entity, changes, { lang: 'en' });

// deleting an entity    👇
Model.delete(entity, { lang: 'en' });

// if you pass a shouldUpdate option to your schema, you can update the contetx's options like below
Schema(
  {},
  {
    shouldUpdate() {
      return { update: true, contextOptionsUpdate: { lang: 'de' } };
    }
  }
);

// 2) within your schema, wherever the operation context is available, you can use the __getOptions__ method of the context to access the options you provided

// in a validator
function validateUsername(value, sumary: Summary<Input, Output, CtxOptions>) {
  const { context } = summary;
  const { lang } = context.__getOptions__();

  // ... further processing

  return true;
}
```

# The Operation Summary

```ts
import type { Context, Summary } from 'ivo';

type Input = {};
type Output = {};

type IContext = Context<Output, Input>;
type ISummary = Summary<Output, Input>;

// 👇 S below is the same as `ISummary`
// 👇 this is what `ISummary` looks like
type S =
  | Readonly<{
      changes: null;
      context: IContext;
      operation: 'creation';
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      operation: 'update';
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>;

const Model = new Schema<Output, Input>(definitions).getModel();

type Handler = (context: IContext) => void | Promise<void>;

type HandlerWithSummary = (summary: ISummary) => void | Promise<void>;
```

## Life Cycle handlers

These are functions that are invoked during a life cycle operation (`creation`, `failure` or `update`)

### onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. These listeners have access to a context without virtauls even if passed to the delete method of the model. Default **[ ]**. They are expected to respect the signature below

```ts
import { DeletionContext } from 'ivo';

// deleting an entity    👇
Model.delete(entity, { lang: 'en' });

// in a delete handler
function onDelete(ctx: DeletionContext<Output, CtxOptions>) {
  const { id, name, __getOptions__ } = ctx;
  const { lang } = __getOptions__(); // { lang: "en" }
}
```

### onFailure

A void function or array of void functions(async / sync) you want to execute every time the **`create`** & **`update`** operations are unsuccessful. Default **[ ]**. They are expected to respect the `type HandlerWithSummary` as shown above

> N.B: They are only allowed on properties that support and have validators

### onSuccess

A void function or array of void functions(async / sync) you want to execute every time the **`create`** & **`update`** operations are successful. Handlers for this event should expect the operation's summary as only parameter. Default **[ ]**. They are expected to respect the `type HandlerWithSummary` as shown above

As from `v2.5.0`, these handlers have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create & update methods of your models.

If the operation is unsuccessful, `data` and `handleSuccess` will be `null`

```js
const { data, error, handleSuccess } = await UserModel.create(userData);

if (data) await handleSuccess();
```
