## isBooleanOk

To validate boolean values

```ts
import { isBooleanOk } from 'ivo';

console.log(isBooleanOk('true')); // { reasons: ["Expected a boolean"], valid: false }

console.log(isBooleanOk(false)); // { valid: true, validated: false }
```
