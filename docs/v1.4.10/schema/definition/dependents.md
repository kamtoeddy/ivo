# Dependent Properties

If set to **`true`**, any external attempt to modify the of the said property will be ignored; making it's value solely modifiable via the life cycle listeners and side effects.

One such property `must` have a default value and can be used in combination with other rules like **readonly**, **shouldInit**, etc but **`cannot be required`**.

Out of the box, dependent is assumed to be false for every property
