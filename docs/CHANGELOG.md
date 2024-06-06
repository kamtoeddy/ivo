# Changelog

All notable changes to this project will be documented in this file.

# v1.4.3 <small><sup>2024-06-03</sup></small>

- Fix bug causing post-validation array to continue runnning after first errors are returned

# v1.4.2 <small><sup>2024-06-03</sup></small>

- Fix crash that occurs when an onSuccess config object has a repeated property in its own properties array

# v1.4.1 <small><sup>2024-06-03</sup></small>

- Add support for onSuccess options as [config objects](./v1.4.0/life-cycles.md#config-objects)

# v1.4.0 <small><sup>2024-05-13</sup></small>

- Post-validators:
  - Add support for sequencial execution
  - Rename `handler` to `handlers`
  - Allow post-validation configs to be subsets of others if they don't have exactly the same properties
- Built-in validators:
  - Rename `isBooleanOk` to [`validateBoolean`](./v1.4.0/validators.md#validateboolean)
  - Rename `isCreditCardOk` to [`validateCreditCard`](./v1.4.0/validators.md#validatecreditcard)
  - Rename `isEmailOk` to [`validateEmail`](./v1.4.0/validators.md#validateemail)
  - Replace `isArrayOk` with [`makeArrayValidator`](./v1.4.0/validators.md#makearrayvalidator)
  - Replace `isNumberOk` with [`makeNumberValidator`](./v1.4.0/validators.md#makenumbervalidator)
  - Replace `isStringOk` with [`makeStringValidator`](./v1.4.0/validators.md#makestringvalidator)

# v1.3.5 <small><sup>2024-05-07</sup></small>

- Make post-validators to run after sanitization of virtuals and resolvement of dependent properties

# v1.3.4 <small><sup>2024-05-06</sup></small>

- Fix bug making post-validators to fail when fieldErrors are return in error object

# v1.3.3 <small><sup>2024-05-06</sup></small>

- Export internal utils & types to help manipulate field errors in validators

# v1.3.2 <small><sup>2024-05-05</sup></small>

- Fix TS typing issue with enumerated readonly properties

# v1.3.1 <small><sup>2024-04-26</sup></small>

- Update type inference of isArrayOk, isNumberOk and isStringOk utilities
- Exported isOneOf utility

# v1.3.0 <small><sup>2024-04-14</sup></small>

- Split Context into ImmutableContext & MutableContext and Summary into ImmutableSummary & MutableSummary
- Add `__updateOptions__` method to MutableContext to help update CtxOptions from within an operation
- Fix issue with post-validation config not rejecting invalid properties
- Fix issue with ctx options not being reset on every operation

# v1.2.3 <small><sup>2024-03-31</sup></small>

- Fix issue with built-in validator casting non-nullable values to string

# v1.2.2 <small><sup>2024-03-28</sup></small>

- Improve type inference of schema definition

# v1.2.1 <small><sup>2024-02-22</sup></small>

- Fixed issue causing secondary validators not to respect `shouldInit` & `shouldUpdate` rules

# v1.2.0 <small><sup>2024-02-22</sup></small>

- Fixed issue with `DeletionContext` being exported as `DeleteContext`
- Cleaned up tests related to post-validation and secondary validations
- Updated docs on secondary validations

# v1.1.0 <small><sup>2024-02-20</sup></small>

- Added support for upto 2 validators per property (Primary & Secondary)

# v1.0.2 <small><sup>2024-02-06</sup></small>

Fixed bugs related to use with cjs

# v1.0.1 <small><sup>2024-02-01</sup></small>

first release

# v0.0.1 <small><sup>2023-12-08</sup></small>

package setup
