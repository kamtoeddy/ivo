# The Operation Context

This is an object comprized of a mix of input and output values of the instance during a life cycle operation ( creation or update ) plus any virtual properties (if present during the operation) defined in your schema.

```ts
import { type Context } from 'ivo';

type Ctx = Context<Input, Output>;
```

## Context Options

This is a way of providing extra information (releted or not related to you schema) to operations like creation, updates and deletion. Some good usecases would be **dependency injection (DI)** and **internationalization (i18n)**

How to use:

```ts
type UserInput = {
  email: string;
  name: string;
};

type User = {
  email: string;
  id: string;
  name: string;
};

interface UserRepo {
  findByEmail: (email: User['email']) => Promise<User | null>;
  //  ... other methods
}

type CtxOptions = {
  lang: 'en' | 'de' | 'fr'; // lang for i18n
  userRepo: UserRepo; // userRepo for DI
};

// 1) define your schema
const Model = new Schema<UserInput, User, CtxOptions>({
  id: { constant: true, value: generateUserId },
  email: {
    required: true,
    async validator(value, { options: { userRepo } }) {
      if (!isEmail(value))
        return { valid: false, reason: 'Invalid email provided' };

      const isEmailTaken = await userRepo.findByEmail(value);

      return isEmailTaken
        ? { valid: false, reason: 'email already taken' }
        : true;
    },
  },
  name: { required: true, validator: validateName },
}).getModel();

// 2) pass it to related operations
import { userRepo } from 'data-access/users';

// creating an entity   👇
Model.create(input, { lang: 'en', userRepo });

// updating an entity             👇
Model.update(entity, changes, { lang: 'en', userRepo });

// deleting an entity    👇
Model.delete(entity, { lang: 'en', userRepo });

// 3) access the context options as below

// in a validator
function validateName(
  value,
  summary: IvoSummary<UserInput, User, CtxOptions>,
) {
  const { options, updateOptions } = summary;
  const { lang } = options;

  // ... further processing

  // update options
  updateOptions({ lang: 'de' });

  return true;
}
```

# The Operation Summary

```ts
import type { Context, IvoSummary, ReadonlyIvoSummary } from 'ivo';

type Input = {};
type Output = {};

type IContext = Context<Input, Output>;
type Summary = IvoSummary<Input, Output, CtxOptions>;

// 👇 S represents is what `Summary` looks like
type S =
  | Readonly<{
      changes: null;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>;
      updateOptions: (updates: Partial<CtxOptions>) => void;
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>;
      updateOptions: (updates: Partial<CtxOptions>) => void;
    }>;
  
type ReadonlySummary = ReadonlyIvoSummary<Input, Output, CtxOptions>;
  
// 👇 Rs represents is what `ReadonlySummary` looks like
type Rs =
  | Readonly<{
      changes: null;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>; // 👇 notice that the `updateOptions` method is missing
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
      options: Readonly<CtxOptions>; // 👇 notice that the `updateOptions` method is missing
    }>;

const Model = new Schema<Input, Output>(definitions).getModel();

type FailureHandler = (ctx: IContext, options: CtxOptions) => void | Promise<void>;

type HandlerWithSummary = (summary: ReadonlySummary) => void | Promise<void>;
```

## Life Cycle handlers

These are functions that are invoked during a life cycle operation (`creation`, `failure` or `update`)

### onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. These listeners have access to a context without virtauls even if passed to the delete method of the model. Default **[ ]**. They are expected to respect the signature below

```ts
// signature
function onDelete(data: Output, options: CtxOptions) {
  const { id, name } = data;
  const { lang } = options; // { lang: "en" }
}

// how to trigger after deleting an entity
Model.delete(entity, { lang: 'en' });

```

### onFailure

A function or array of functions(async / sync) you want to execute every time the **`create`** & **`update`** operations are unsuccessful. Default **[ ]**.

> N.B: They are only allowed on properties that support and have validators.

These handlers have to be triggered manually by invoking the handleFailure method of the operation's results object returned by the create & update methods of your models.

> If the operation is successful, `error` and `handleFailure` will be `null`

```js
// signature
function onFailure(ctx: IContext, options: CtxOptions) {
  const { id, name } = ctx;
  const { lang } = options; // { lang: "en" }
}


const { error, handleFailure } = await UserModel.create(userData);

// how to trigger after a validation error
if (error) await handleFailure();
```

### onSuccess

A function, [config object](#config-objects) or array of config objects or functions(async / sync) you want to execute every time the **`create`** & **`update`** operations are successful. Handlers for this event should expect the operation's summary as only parameter. Default **[ ]**. Handlers are expected to respect the `type HandlerWithSummary` as shown above.

These handlers have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create & update methods of your models.

> N.B: If the operation is unsuccessful, `data` and `handleSuccess` will be `null`

```js
// signature
function onSuccess(summary: Summary) {
  const { ctx, options } = summary;
  const { id, name } = ctx;
  const { lang } = options; // { lang: "en" }
}


const { data, error, handleSuccess } = await UserModel.create(userData);

// how to trigger after successful validation
if (data) await handleSuccess();
```

#### Config objects

These were introduced in version 1.4.1 to allow for more simplicity and flexibility when dealing with success handlers related to more than one property. A success config object should have the following shape:

```ts
type ConfigObject = {
  properties: ArrayOfMinSizeTwo<keyof (Input & Output)>;
  handler: HandlerWithSummary | HandlerWithSummary[];
};
```

Example:

```ts
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: handler, // the handler will be executed during all success operations
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [handler1, handler2], // the handlers will be executed during all success operations
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: {
    properties: ['email', 'name'],
    handler, // always executed at creation during updates with either email or name
  },
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: {
    properties: ['email', 'name'],
    handler: [handler1, handler2], // always executed at creation during updates with either email or name
  },
});

// or
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    handler1, // executed during all success operations
    { properties: ['id', 'email'], handler: handler2 },
    { properties: ['firstName', 'lastName'], handler: [handler3, handler4] },
  ],
});

// ✅ as from v1.5.1 you can provide subsets of other configs
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    { properties: ['id', 'email', 'firstName'], handler: handler2 },
    { properties: ['email', 'firstName'], handler: [handler3, handler4] },
  ],
});

// ❌ this is not allowed
const Model = new Schema<Input, Output>(definitions, {
  onSuccess: [
    { properties: ['id', 'email'], handler: [handler1, handler2] },
    { properties: ['email', 'id'], handler: handler3 },
  ],
});
```
