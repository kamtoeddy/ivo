# Structure of Schema Error

As stated earlier, the create and update methods may throw errors. They will, if the data passed are invalid at creation, cloning and updates

```ts
type SchemaError = {
  message: string; // e.g. Validation Error
  payload: {
    [key: string]: string[]; // e.g. name: ["Invalid name", "too long"]
  };
  statusCode: number; // e.g. 400
};
```
