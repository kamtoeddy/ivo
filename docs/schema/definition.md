# Defining a schema

The Schema constructor accepts 2 properties:

- definitions:
  - **This is a required property**
  - This is object contains all the definitions of the structure of your data
  - See illustration above **@Defining a model**
- options
  - This is optional
  - An object composed of the following optional properties:
    1. **timestamp**
       - boolean which tells clean-schema whether or not to add createdAt and updatedAt to instances of your model
       - default: false

```javascript
const adminSchema = new Schema({ ...definitions }, { timestamp: true });
```

# Properties of a _definition object_

```javascript
// the value of id in the definitions object below
// is the definition object
const definitions = {
  id: {
    readonly: true,
    validator: validateId,
  },
};

const userSchema = new Schema({ ...definitions });
```

| Property   | Type     | Description                                                                                                                                                                                        |
| ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| default    | any      | A value of any type you wish to use for a given property. Default **undefined**                                                                                                                    |
| dependent  | boolean  | If set to true, clean-schema will prevent any external modification of the property; making it's value soley dependent on another property via the onCreate / onUpdate handlers. Default **false** |
| onCreate   | array    | An array of functions(async / sync) you want to be executed when an instance of your model gets created. Default **[ ]**                                                                           |
| onUpdate   | array    | An array of functions(async / sync) you want to be executed when a particular property of your instance get updated. Default **[ ]**                                                               |
| readonly   | boolean  | If true will be required at initialization and will never allow updates. If true with shouldInit: false, will not be initialized but allowed to update only once. Default **false**                |
| required   | boolean  | Specifies a property that must be initialised. Default **false**                                                                                                                                   |
| sideEffect | boolean  | Used with onUpdate to modify other properties but is not attached to instances of your model. Default **false**                                                                                    |
| shouldInit | boolean  | Tells clean-schema whether or not a property should be initialized. Default **true**                                                                                                               |
| validator  | function | A function(async / sync) used to validated the value of a property. Must return {reason:string, valid: boolean, validated: undefined or any}. Default **null**                                     |

> N.B: Clean-schema will throw an error if the no property is defined or if none of the properties defined are valid.
