# Archived Schemas

These schemas are extended versions of your schemas that have validators and life cycle handlers on individual properties turned off.

Archived schemas cannot be further extended. They return a model that has 2 factory methods, `create` & `delete`. The idea here is that archived data should not be updated

```ts
import { Schema } from "clean-schema"

const postSchema = new Schema({...});

const archivedSchema = postSchema.getArchivedSchema(options);

const ArchivedModel = archivedSchema.getModel();
```

## Options

The `getArchivedSchema` method accepts an object as only optional argument. This object represents the options that should be applied to the archived schema.

- `archivedAt`: A string to serve as the label for the date the archived document was created. This label cannot be the same as an existing property or timestamp on the original schema
- `onDelete`: A function or an array of functions that would be triggered when the archived entity gets deleted (Just like [delete handlers](./definition/life-cycles.md#ondelete))
- `onSuccess`: A function or an array of functions that would be triggered when the archived entity gets created. These handlers should match the signature of the `onDelete` handlers above because Archived models only support creation & deletion

## Archived Models

Unlike normal models, an archived model supports only 2 operations; `create` & `delete`

- `ArchivedModel.create` expects one argument of type `Input` and returns an object with properties `data` & `handleSuccess` just like when creating entities with normal Models. There is no validation here so this method is synchronous, hence no need to await the results
- `ArchivedModel.delete` expects one argument of type `Output`. Just like the delete method of normal models, this tiggers all 'onDelete' handlersm of your schema

> N.B: The only form of validation performed by these methods is to make sure that the data provided respects the structure of your schema. This means that not providing a value would result in your context having undefined for that value. Also, providing an invalid value for a property would go undetected
