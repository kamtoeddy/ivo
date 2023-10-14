# Changelog

All notable changes to this project will be documented in this file.

# v4.0.0 <small><sup>[x][x]-10-2023</sup></small>

Breaking Changes

- [Update] change order of `Input` & `Output` types passed to Schema constructor

New Features

- [Add] [`__getOptions__`](./v4.0.0/life-cycles.md#context-options) method to operation context
- [Add] [`errorTool`](./v4.0.0/index.md#errortool) to option to help with custom validation errors
- [Add] [`useParentOptions`](./v4.0.0/index.md#useparentoptions) to options of extended schemas to opt in/out of parent options

Other Updates

- [Update] Update SchemaError by remove statusCode
- [Update] make `dependent: true` rule in schema definition optional. Dependent properties only need `dependsOn` & a `resolver` function
- [Doc] update docs & broken links
- [Fix] numerous bug fixes

# v3.5.3 <small><sup>21-09-2023</sup></small>

- [Update] made sure the `alias` kayword if only allowed on virtual properties

# v3.5.2 <small><sup>13-09-2023</sup></small>

- [Add] added [`equalityDepth`](./v3.5.2/index.md#equalitydepth) to schema options
- [Docs] fix broken links in various sections of docs

# v3.5.1 <small><sup>13-08-2023</sup></small>

- [Docs] fix broken links in section on how to define a schema
- [Fix] fix issue with isStringOk rejecting enums which are readonly arrays

# v3.5.0 <small><sup>06-08-2023</sup></small>

- [Add] added [`setMissingDefaultsOnUpdate`](./v3.5.0/index.md#setmissingdefaultsonupdate) to schema options
- [Update] updated types of `validate.isStringOk`

# v3.4.0 <small><sup>04-08-2023</sup></small>

- [Update] changed the signature of schemas

# v3.3.1 <small><sup>02-08-2023</sup></small>

- [Update] improved typescript types

# v3.3.0 <small><sup>01-08-2023</sup></small>

- [Add] added support for [`global shouldUpdate`](./v3.3.0/schema/definition/index.md#shouldupdate) rule
- [Update] the create, clone and update methods of models now return `null` instead of `undefined`
- [Update] made sure that the various messages of the Schema error are properly typed
- [Update] updated signature of `operation summary` so it holds `null` for instead of `undefined` at creation. [see more](./v3.3.0/schema/definition/life-cycles.md#the-operation-summary)
- [Remove] removed archived schemas

# v3.2.0 <small><sup>02-05-2023</sup></small>

- [Update] completed work on [`archived schemas`](./v3.2.0/schema/archived-schemas.md)

# v3.2.0-beta.0 <small><sup>26-04-2023</sup></small>

Breaking Changes

- [Update] changed the signature and behaviour of resolvers of [dependent properties](./v3.2.0/schema/definition/dependents.md)
- [Update] changed the signature and behaviour of [life cycle handlers](./v3.2.0/schema/definition/life-cycles.md#life-cycle-listeners)
- [Update] changed second argument of `validator functions` from operation context to operation summary [more](./v3.2.0/validate/index.md#validators)
- [Add] added a 'trim' option to validate.isStringOk so one can opt in and out of trimmed values [more](./v3.2.0/validate/isStringOk.md)

Features

- [Add] added support for [`aliases`](./v3.2.0/schema/definition/virtuals.md#aliases) on virtuals
- [Add] added support for [`global onDelete`](./v3.2.0/schema/definition/index.md#ondelete) in schema options
- [Add] added support for [`global onSuccess`](./v3.2.0/schema/definition/index.md#onsuccess) in schema options
- [Add] added utility types [`Context` & `Summary`](./v3.2.0/schema/definition/life-cycles.md#life-cycle-listeners) for TypeScript
- [Add] added support for [`otherReasons`](./v3.2.0/validate/index.md) in validators
- [Add] added support for [`archived schemas and models`](./v3.2.0/schema/archived-schemas.md)

# v3.1.0 <small><sup>05-02-2023</sup></small>

Breaking Changes

- [Update] changed `sideEffect` rule to `virtual`

# v3.0.7 <small><sup>05-02-2023</sup></small>

- [Update] removed unused methods from SchemaCore
- [Fix] updated types of setters of 'shouldInit' & 'shouldUpdate' rules so that they do not suggest the operation's name anymore

# v3.0.6 <small><sup>02-02-2023</sup></small>

- [Fix] fixed problem with 'requiredError' rule still being suggested as a configuration when defining a schema

# v3.0.5 <small><sup>02-02-2023</sup></small>

- [Fix] fixed problem of types not being properly inferred for updated values

# v3.0.4 <small><sup>01-02-2023</sup></small>

- [Fix] fix problems whereby newly resolved values of dependent properties were considered as updates even though their values were not different from the previous values
- [Fix] fix problems with types when using 'sanitize' rule with TS
- [Update] updated how values are set internally so that values of properties sideEffects are only accepted at creation, cloning, deletion and as changes during updates. Timestamps are only accepted as current values during updates & deletion

# v3.0.3 <small><sup>31-01-2023</sup></small>

- [Fix] fix problems with types when using 'sanitize' rule with TS

# v3.0.2 <small><sup>31-01-2023</sup></small>

- [Fix] fix various type issues with TS

# v3.0.1 <small><sup>31-01-2023</sup></small>

- [Fix] fix problems with types when using 'sanitize' rule with TS

# v3.0.0 <small><sup>31-01-2023</sup></small>

Breaking Changes

- [Update] updated the definition of [dependent properties](./schema/definition/dependents.md)
- [Update] updated the definition of [side effect properties](./schema/definition/side-effects.md)
- [Update] updated how schema inheritance works [more](./schema/definition/inheritance.md)
- [Update] updated how conditionally required properties work [more](./schema/definition/required.md#required-by)
- [Remove] removed `requiredError` rule
- [Remove] removed `onChange`, `onCreate` & `onUpdate` rules

Updates

- [Add] possibility to use one timestamp value at a time. [more](./v3.0.0/schema/definition/index.md#timestamps)
- [Add] 'shouldUpdate' rule [more](./v3.0.0/schema/definition/index.md#accepted-rules)
- [Update] improved type inference during schema inheritance
- [Update] upgraded TS to v4.9.4
- [Update] rewrote types (to reduce files)

Bug Fixes

- [Fix] fixed problem of Model.validate method still holding values from previous operation in ctx
- [Fix] fixed problem with Model.validate(prop, value) does not return the value provided when prop's validator does not return a validated(sanitized) value
- [Fix] fixed problem with schema not complaining about invalid rules like 'yoo'

- [Fix] fixed issue with some conditionally required fields not being called at when another conditionally required field was required during the same operation

# v2.6.0 <small><sup>08-01-2023</sup></small>

- [Update] updated the way validator responses should behave

# v2.5.16 <small><sup>05-01-2023</sup></small>

- [Fix] fixed problem of context of operation of onSuccess listeners missing some values
- [Fix] fixed problem of model allowing default timestamp values(`createdAt` & `updatedAt`) to be set as values when `schema.options.timestamps` config is in default mode

# v2.5.15 <small><sup>22-12-2022</sup></small>

- [Update] performance enhancements

# v2.5.14 <small><sup>22-12-2022</sup></small>

- [Fix] fixed problem of constants not being assigned to finalContext

# v2.5.13 <small><sup>22-12-2022</sup></small>

- [Update] made onSuccess listeners of sideEffect properties to be respected at creation, cloning & during updates

# v2.5.12 <small><sup>19-12-2022</sup></small>

- [Add] support for callable 'shouldInit'

# v2.5.11 <small><sup>12-12-2022</sup></small>

- [Update] made `validate.isArrayOk` to accept async filter & modifier methods

# v2.5.10 <small><sup>11-12-2022</sup></small>

- [Add] Concerned life cycle as second parameter of onChange listeners

# v2.5.9 <small><sup>11-12-2022</sup></small>

- [Add] Concerned life cycle as second parameter of onSuccess listeners

# v2.5.8 <small><sup>06-12-2022</sup></small>

- [Update] Schema validations message for errors

# v2.5.7 <small><sup>04-12-2022</sup></small>

- [Add] Validation checks for values passed to clone, create, delete and update methods of models
- [Update] Improved performance resolving onDelete listeners

# v2.5.6 <small><sup>02-12-2022</sup></small>

- [Update] Improved performance with onFailure listeners
- [Docs] Updated README

# v2.5.5 <small><sup>28-11-2022</sup></small>

- [Update] Improved performance by making execution of life cycle listeners further parallel
- [Fix] fixed bug with readonly properties being modified vialife cycle listeners after having changed

# v2.5.4 <small><sup>20-11-2022</sup></small>

- [Docs] fixed broken links in README

# v2.5.3 <small><sup>05-11-2022</sup></small>

- [Update] enforced readonly nature of operation contexts

# v2.5.2 <small><sup>01-11-2022</sup></small>

- [Fix] fixed type errors with shouldInit rule

# v2.5.1 <small><sup>01-11-2022</sup></small>

- [Fix] fixed type errors with validate.isEmailOk

# v2.5.0 <small><sup>28-10-2022</sup></small>

- [Update] made onSuccess listeners to be run via the handleSuccess function returned from create, clone & update operations

# v2.4.2 <small><sup>16-10-2022</sup></small>

- [Fix] fixed readonly lax properties with callable defaults that return dynamic values not being readonly

# v2.4.1 <small><sup>13-10-2022</sup></small>

- [Update] Slight performance improvements

# v2.4.0 <small><sup>08-10-2022</sup></small>

- [Update] made it possible to have async setter functions for constants values
- [Docs] Updated broken link to details of constant properties in `v2.1.0`

# v2.3.0 <small><sup>05-10-2022</sup></small>

- [Update] made it possible to have onSuccess listeners on constants

# v2.2.0 <small><sup>05-10-2022</sup></small>

- [Update] made it possible to have onDelete listeners on constants

# v2.1.1 <small><sup>20-09-2022</sup></small>

- [Update] improved on internal performance by caching lax props

# v2.1.0 <small><sup>20-09-2022</sup></small>

- [Add] mode.delete method
- [Add] onDelete, onFailure & onSuccess life cycles
- [Update] sideEffects can now be requiredBy
- [Fix] fixed dependent props not being set during cloning

# v2.0.1 <small><sup>18-09-2022</sup></small>

- [Update] did clean-ups

# v2.0.0 <small><sup>18-09-2022</sup></small>

- [Add] Schema.options.errors
- [Changed] the results of create, clone & update operations
- [Fix] fixed lax properties being required during cloning

# v1.5.0 <small><sup>09-09-2022</sup></small>

- [Add] constant properties
- [Add] callable required properties
- [Docs] updated broken links in docs files

# v1.4.11 <small><sup>08-09-2022</sup></small>

- [Fix] fixed model.update not ignoring properties that haven't changed

# v1.4.10 <small><sup>07-09-2022</sup></small>

- [Docs] made docs more comprehensive
- [Remove] removed ApiError from dist exports

# v1.4.6 <small><sup>04-09-2022</sup></small>

- [Fix] fixed life cycle listeners being executed even on invalid operations
- [Docs] updated docs folder structure and readme

# v1.4.5 <small><sup>02-09-2022</sup></small>

- [Fix] fixed custom timestamp values being limited to provided interface

# v1.4.4 <small><sup>02-09-2022</sup></small>

- [Update] typescript support

# v1.4.3 <small><sup>02-09-2022</sup></small>

- [Update] overall typescript support
- [Update] ApiError
- [Add] typescript interfaces for listeners & validators

# v1.4.2 <small><sup>31-08-2022</sup></small>

- [Add] possibility to return single reason from validators
- [Add] tests for user defined validation errors

# v1.4.1 <small><sup>27-08-2022</sup></small>

- [Update] did cleanups

# v1.4.0 <small><sup>27-08-2022</sup></small>

- [Change] replaced makeModel by schema.getModel\<I>() method

# v1.3.11 <small><sup>27-08-2022</sup></small>

- [Update] enforced definition checks for readonly(lax) + validator & no default
- [Update] isCreditCardOk now validates strings to be trimmed
- [Docs] updated docs on isNumberOk for string parsing
- [Docs] updated docs on isArrayOk (options.sortOrder)
- [Docs] updated changelog

# v1.3.10 <small><sup>25-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.9 <small><sup>24-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.8 <small><sup>24-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.7 <small><sup>19-08-2022</sup></small>

- [Fix] fixed lax properties behaving as required

# v1.3.6 <small><sup>18-08-2022</sup></small>

- [Add] possibility to use readonly as "lax"
- [Add] timestamp props to context

# v1.3.5 <small><sup>14-08-2022</sup></small>

- [Update] Performance optimizations

# v1.3.4 <small><sup>12-08-2022</sup></small>

- [Add] Support for single life cycle listeners
- [Docs] Updated docs

# v1.3.3 <small><sup>09-08-2022</sup></small>

- [Remove] removed \_isError from ApiError

# v1.3.2 <small><sup>06-08-2022</sup></small>

- [Add] more tests & typescript type annotations

# v1.3.1 <small><sup>05-08-2022</sup></small>

- [Add] extra checks to ensure proper validation during updates

# v1.3.0 <small><sup>05-08-2022</sup></small>

- [Add] added onChange property for both create & update events
- [Docs] updated docs for v1.2.5 & v1.2.6
- [Remove] support for onUpdate rule on side effect properties

# v1.2.6 <small><sup>04-08-2022</sup></small>

- [Update] added check for non-empty array for onCreate and onUpdate listeners

# v1.2.5 <small><sup>03-08-2022</sup></small>

- [Update] did parallellization of schema validation at creation & updates

# v1.2.4 <small><sup>21-08-2022</sup></small>

- [Clean up] Stopped typescript from transpiling tests
- [Docs] added link to validate.isCreditCardOk in readme.md

# v1.2.3 <small><sup>21-08-2022</sup></small>

- [Add] built-in validator validate.isCreditCardOk
- [Add] CI pipeline via github actions
- [Docs] Added docs for isCreditCardOk

# v1.2.2 <small><sup>21-07-2022</sup></small>

- [Docs] Updated link to changelog in README.md

# v1.2.1 <small><sup>21-07-2022</sup></small>

- [Fix] Side effect properties not being initialised

# v1.2.0 <small><sup>20-07-2022</sup></small>

- [Remove] Schema options.timestamp so only options.timestamps is supported

# v1.1.12 <small><sup>20-07-2022</sup></small>

- [Add] Possibility to pass a string/string[] as options when cloning an entity
- [Docs] Updated docs on structure of validator functions
- [Docs] Updated some errors in docs
- [Remove] Schema options.timestamp so only options.timestamps is supported

# v1.1.11 <small><sup>19-07-2022</sup></small>

- [Fix] error: Exported variable 'YourModel' has or is using name 'Model' from external module "clean-schema" but cannot be named

# v1.1.10 <small><sup>19-07-2022</sup></small>

- [Add] generics for typescript

# v1.1.9 <small><sup>19-07-2022</sup></small>

- [Fix] invalid properties crashing create & updates properly

# v1.1.8 <small><sup>19-07-2022</sup></small>

- [Fix] non readonly|required not being initialized
- [Fix] updates not working properly

# v1.1.7 <small><sup>18-07-2022</sup></small>

- [Add] Updated type definitions of onCreate/onUpdate listeners for typescript

# v1.1.6 <small><sup>18-07-2022</sup></small>

- [Add] Updated type definitions for typescript support

# v1.1.5 <small><sup>18-07-2022</sup></small>

- [Fix] Fixed issue with clean-schema requiring options parameter on Schema constructor

# v1.1.4 <small><sup>18-07-2022</sup></small>

- [Fix] Fixed issue with clean-schema requiring options parameter on SchemaCore constructor

# v1.1.3 <small><sup>18-07-2022</sup></small>

- [Deprecated] Changed schema options.timestamp to timestamps
- [Fix] Schema options still accepting timestamps with keys already on schema
- [Docs] Updated docs of how to use [validator.isEmailOk](./v1.4.6/validate/isEmailOk.md#validateisemailok)

# v1.1.2 <small><sup>17-07-2022</sup></small>

- [Docs] Added Changelog

# v1.1.1 <small><sup>17-07-2022</sup></small>

- [Add] Possibility to use custom regular expression with [validator.isEmailOk](./v1.4.6/validate/isEmailOk.md#validateisemailok)

- [Fix] Internal mechanism in charge of data manipulation which would cause bugs

# v1.1.0 <small><sup>17-07-2022</sup></small>

- [Docs] Imporved details of invalid schema error for onCreate & onUpdate methods

# v1.0.35 <small><sup>16-07-2022</sup></small>

- [Add] Built-in validator for emails [validator.isEmailOk](./v1.4.6/validate/isEmailOk.md#validateisemailok)
- [Docs] Improved details of invalid schema error

# v1.0.34 <small><sup>15-07-2022</sup></small>

- [Add] Possibility to override built in property names for timestamps
