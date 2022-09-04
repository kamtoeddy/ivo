## validate.isBooleanOk

To validate boolean values

```javascript
const { validate } = require("clean-schema");

console.log(validate.isBooleanOk("true")); // { reasons: ["Expected a boolean"], valid: false, validated: undefined }

console.log(validate.isBooleanOk(false)); // { reasons: [], valid: true, validated: false }
```
