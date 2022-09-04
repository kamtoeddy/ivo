## validate.isCreditCardOk

A tiny utility method to test if a credit/debit **`Card Number`** is valid; not the credit card itself

```js
const { validate } = require("clean-schema");

console.log(validate.isCreditCardOk(""));
// {reasons: ["Invalid card number"], valid: false, validated: undefined}

console.log(validate.isCreditCardOk(5420596721435293));
// {reasons: [], valid: true, validated: 5420596721435293}

console.log(validate.isCreditCardOk("5420596721435293"));
// {reasons: [], valid: true, validated: "5420596721435293"}
```

It returns:

```ts
interface ResponseStructure {
  reasons: string[]; // [] or ["Invalid card number"],
  valid: boolean;
  validated: number | string | undefined;
}
```
