## isCreditCardOk

A tiny utility method to test if a credit/debit **`Card Number`** is valid; not the credit card itself

```ts
import { isCreditCardOk } from 'ivo';

console.log(isCreditCardOk(''));
// { reasons: ["Invalid card number"], valid: false }

console.log(isCreditCardOk(5420596721435293));
// { valid: true, validated: 5420596721435293}

console.log(isCreditCardOk('5420596721435293'));
// { valid: true, validated: "5420596721435293"}
```

It returns:

```ts
type ValidationResponse =
  | { reasons: string[]; valid: false }
  | { valid: true; validated: number | string };
```
