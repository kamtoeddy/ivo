# The Operation Context

This is an object comprized of values of the instance during a life cycle operation ( cloning, creation or update ) plus any side effect values (if present during the operation) defined in your schema.

# Life Cycle listeners

These are functions that are invoked during a life cycle operation and recieve the [operation context](#the-operation-context) as only parameter. They are expected to have the structure of the `onComplete function` below

```ts
type Listener<T> = (
  ctx: T
) => Partial<T> | Promise<Partial<T>> | void | Promise<void>;
```

Example:

```js
const transactionSchema = new Schema({
  completedAt: {
    default: "",
    dependent: true,
    readonly: true,
    onChange: onCompletedAt,
  },
  isComplete: {
    default: false,
    readonly: true,
    shouldInit: false,
    onUpdate: onIsComplete,
    validator: (val) => validateBoolean(val),
  },
});

// destructuring isComplete from the validation context
function onIsComplete({ isComplete }) {
  return { completedAt: isComplete ? new Date() : "" };
}

function onCompletedAt({ completedAt, isComplete }) {
  console.log(completedAt, isComplete); // new Date() true
}
```

> If the listener (handler) does not return an object with the schema's properties or side effect properties, the value returned is simply ignored.

## onChange

A function or array of functions(async / sync) you want to execute every time an instance of your model gets created or updated. **`NB:`** `These listeners(onChange) are always executed after onCreate & onUpdate listeners of the same property.` Default **[ ]**

## onCreate

A function or array of functions(async / sync) you want to execute every time an instance of your model gets created. Default **[ ]**

## onDelete

A void function or array of void functions(async / sync) you want to execute every time an instance of your model gets deleted. That is; every time the **`model.delete`** method is invoked. Default **[ ]**

## onFailure

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are unsuccessful. Default **[ ]**

## onSuccess

A void function or array of void functions(async / sync) you want to execute every time the **`create`**, **`clone`** & **`update`** operations are successful. Default **[ ]**

## onUpdate

A function or array of functions(async / sync) you want to execute every time the property defined on get updated. Default **[ ]**
