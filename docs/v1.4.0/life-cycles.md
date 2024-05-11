# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( creation or update ) plus any virtual properties (if present during the operation) defined in your schema.

This object also has a method `__getOptions__` that return a readonly copy of the context options provided at creation, updates and deletions. [more here](#context-options)

```ts
import type { ImmutableContext, MutableContext } from 'ivo';

type Input = { virtualProp: string };
type Output = { constantProp: string; dependentProp: string };
type CtxOptions = { lang: 'de' | 'en' | 'fr' };

type IContext = ImmutableContext<Input, Output, CtxOptions>;
type MContext = MutableContext<Input, Output, CtxOptions>;

// 👇 this is what `IContext` above would look like
type IC = {
  __getOptions__: () => Readonly<CtxOptions>;
  constantProp: string;
  dependentProp: string;
  virtualProp: string;
};

// 👇 this is what `MContext` above would look like
type MC = {
  __getOptions__: () => Readonly<CtxOptions>;
  __updateOptions__: (updates: Partial<CtxOptions>) => void;
  constantProp: string;
  dependentProp: string;
  virtualProp: string;
};
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
    async validator(value, { context }) {
      if (!isEmail(value))
        return { valid: false, reason: 'Invalid email provided' };

      const { userRepo } = context.__getOptions__();

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

// 3) within your schema, wherever the operation context is available,
//  you can use the __getOptions__ method of
// the context to access the options you provided

// in a validator
function validateName(
  value,
  summary: MutableSummary<UserInput, User, CtxOptions>,
) {
  const { context } = summary;
  const { lang } = context.__getOptions__();

  // ... further processing

  context.__updateOptions__({ lang: 'de' });

  return true;
}
```

# The Operation Summary

```ts
import type { ImmutableContext, ImmutableSummary } from 'ivo';

type Input = {};
type Output = {};

type IContext = ImmutableContext<Input, Output, CtxOptions>;
type ISummary = ImmutableSummary<Input, Output, CtxOptions>;

// 👇 S below is the same as `ISummary`
// 👇 this is what `ISummary` looks like
type S =
  | Readonly<{
      changes: null;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: false;
      previousValues: null;
      values: Readonly<Output>;
    }>
  | Readonly<{
      changes: Partial<Readonly<Output>>;
      context: IContext;
      inputValues: Partial<Input>;
      isUpdate: true;
      previousValues: Readonly<Output>;
      values: Readonly<Output>;
    }>;

const Model = new Schema<Input, Output>(definitions).getModel();

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

These handlers have to be triggered manually by invoking the handleSuccess method of the operation's results object returned by the create & update methods of your models.

> N.B: If the operation is unsuccessful, `data` and `handleSuccess` will be `null`

```js
const { data, error, handleSuccess } = await UserModel.create(userData);

if (data) await handleSuccess();
```
