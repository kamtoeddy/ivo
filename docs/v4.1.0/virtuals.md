## Virtual Properties

These properties are used to manipulate dependent properties at the level of your model but won't appear on instances, hence don't go to you database.

- They (virtuals) must have:

  - `virtual: true`
  - A validator and
  - Atleast one property that depends on it

- They can have (**`shouldInit === false`**) or `shouldInit` as a function
- They can have (**`shouldUpdate === false`**) or `shouldUpdate` as a function
- They can have `required` as a function
- They can have [aliases](#aliases)
- They can have [errorWithAliasOnly](#error-with-alias-only)
- They can have [sanitizers](#sanitizer)
- They **CANNOT** be dependent, defaulted, strictly required nor readonly

> Out of the box, virtual is **`false`**

Example:

```ts
import { Schema } from 'clean-schema';

type UserInput = {
  blockUser?: boolean;
};

type User = {
  isBlocked: boolean;
};

// definition
const User = new Schema<UserInput, User>({
  blockUser: { virtual: true, validator: validateBoolean },
  isBlocked: {
    default: false,
    dependent: true,
    dependsOn: 'blockUser',
    resolver: ({ context: { blockUser } }) => blockUser
  }
}).getModel();

function validateBoolean(value) {
  if (![false, true].includes(value))
    return { valid: false, reason: `${value} is not a boolean` };
  return { valid: true };
}

// creating
const user = await User.create({ blockUser: true, name: 'Peter' });

console.log(user); // { isBlocked: true }
```

The results of the above operation is an object with a single property `isBlocked`. `name` is missing because it does not belong to our schema but `blockUser` is missing because it is virtual and because it was provided, the value of isBlocked is true instead of the default(false).

The same concept applies to `clone` and `update` operations.

## Aliases

An alias is an extra **external** name for a virtual property. This means that a virtual property with an alias can be accessed by it's name or it's alias. It's that simple 😊

### How to define an alias

- Only virtuals can have aliases
- an alias must be of type `string`
- cannot be the name of another property or virtual on your model (except if the alias is the name of a dependent property on that virtual)
- for best results with TS, the type definitions provided should correspond for your alias and it's virtual property (see in example 1 below)

### Examples

Example 1: Alias with name of related dependent property

```ts
type Input = {
  _virtualQuantity?: number;
};

type Output = {
  quantity: number;
};

type Aliases = {
  quantity: number;
};

const StoreItem = new Schema<Input, Output, Aliases>({
  quantity: {
    default: 0,
    dependent: true,
    dependsOn: '_virtualQuantity',
    resolver: ({ context: { _virtualQuantity } }) => _virtualQuantity
  },
  _virtualQuantity: {
    alias: 'quantity',
    vitual: true,
    validator: validateVirtualQuantity
  }
}).getModel();

// this
const { data: item1 } = await StoreItem.create({ _virtualQuantity: 100 });

// is the same as this
const { data: item2 } = await StoreItem.create({ quantity: 100 });

console.log(item1, item2); // { quantity: 100 } { quantity: 100 }
```

If the virtual and the alias are provided at the same time, the last value is considered

```ts
const { data: item1 } = await StoreItem.create({
  quantity: 20,
  _virtualQuantity: 100
});

const { data: item2 } = await StoreItem.create({
  _virtualQuantity: 11,
  quantity: 5
});

console.log(item1, item2); // { quantity: 100 } { quantity: 5 }
```

Example 2: Alias with unrelated name

```ts
const StoreItem = new Schema({
  quantity: {
    default: 0,
    dependent: true,
    dependsOn: '_virtualQuantity',
    resolver: ({ context: { _virtualQuantity } }) => _virtualQuantity
  },
  _virtualQuantity: {
    alias: 'qty',
    vitual: true,
    validator: validateVirtualQuantity
  }
}).getModel();

// this
const { data: item1 } = await StoreItem.create({ _virtualQuantity: 100 });

// is the same as this
const { data: item2 } = await StoreItem.create({ qty: 100 });

console.log(item1, item2); // { quantity: 100 } { quantity: 100 }
```

> N.B: Virtual properties can only be accessed with their aliases outside of your schema. This is to say that virtual properties should not be accessed on [`operation contexts`](../life-cycles.md#the-operation-context) with their aliases

Aliases are available on the `clone`, `create`, `update` and `validate` methods of your models

## Error with alias only

If a virtual property has an alias and `errorWithAliasOnly` is set to `true`, validation errors for this property will use the alias or the virtual property key as error key depending on which was provided. If both or none are provided, the alias takes precedence. By defaut, this value is `true`

### Examples

```js
import { Schema } from 'clean-schema';

// default: errorWithAliasOnly === true
const Model = new Schema({
  dependentProp: {
    default: null,
    dependsOn: 'virtualProp',
    resolver: () => ''
  },
  virtualProp: { alias: 'alias', virtual: true, validator: validator }
}).getModel();

const { error } = await Model.create();

console.log(error);
// {
//   message: 'VALIDATION_ERROR',
//   payload: { alias: { reasons: ['validation failed'] } }
// }

// errorWithAliasOnly === false
const Model = new Schema({
  dependentProp: {
    default: null,
    dependsOn: 'virtualProp',
    resolver: () => ''
  },
  virtualProp: {
    alias: 'alias',
    errorWithAliasOnly: false,
    virtual: true,
    validator: validator
  }
}).getModel();

const { error } = await Model.create();

console.log(error);
// {
//   message: 'VALIDATION_ERROR',
//   payload: {
//     alias: { reasons: ['validation failed'] },
//     virtualProp: { reasons: ['validation failed'] }
//   }
// }
```

## Sanitizer

This should be used when your virtual property may exist in more than one form. This function is executed immediately the validation step is complete. This function could be synchronous or asynchronous and has access to only one argument, the [operation summary](../life-cycles.md#the-operation-summary)

A good usecase would be when a dealing with file uploads. The example below shows how you could upload a file to a file or cloud storage, get the metadata you'll need to persist as metadata. After sanitization, the resolver of properties that depend (`metadata` in our case) on the these virtuals are run with the new values of the virtual properties

```ts
import { Schema, type Summary } from 'clean-schema';

type FileMetadata = { size: number; url: string };

type Input = {
  file: File | FileMetadata;
  name: string;
};

type Output = {
  id: string;
  metadata: FileMetadata;
  name: string;
};

const FileModel = new Schema<Input, Output>({
  id: { constant: true, value: generateID },
  metadata: {
    default: { size: 0, url: '' },
    dependent: true,
    dependsOn: 'file',
    resolver({ context: { file } }) {
      return file as FileMetadata;
    }
  },
  name: { required: true, validator: validateName },
  file: {
    vitual: true,
    sanitizer: sanitizeFile,
    validator: validateFile
  }
}).getModel();

async function sanitizeFile({ context: { file } }: Summary<Input, Output>) {
  // upload file
  const { size, url } = await uploadFile(file);

  return { size, url } as FileMetadata;
}
```
