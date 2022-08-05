## Schema Inheritance

For any schema that inherits from another, call the extend method on the schema before creating the model. The extend method takes 2 arguments:

1. parent: the schema to inherit from
1. options: an options object with
   - remove: an array of properties to ignore from the parent schema. To override a property, you just do it in the property definitions

```js
const definitions = {
  securePass: {
    required: true,
    validator: validateId,
  },
};

const options = { timestamp: true };

const adminSchema = new Schema(definitions, options).extend(userSchema, {
  remove: ["dob"],
});
```
