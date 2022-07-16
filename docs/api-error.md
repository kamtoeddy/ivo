# Structure of ApiError

As stated earlier, the create and update methods may throw errors. They will, if the data passed are invalid.

1. ### With model({...values}).create()

   - This will happen if any of the values passed were invalid

1. ### With model({...values}).update({...updates})
   - This will happen if any of the updates passed were invalid,
   - if none of the values passed are different from the actual values

```typescript
ApiError: {
  _isError: boolean, // always true
  message: string, // e.g. Validation Error
  payload: {
    [key: string]: string[] // e.g. name: ["Invalid name", "too long"]
  },
  statusCode: number // e.g. 400
}
```
