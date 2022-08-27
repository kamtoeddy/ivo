# Changelog

All notable changes to this project will be documented in this file.

# v1.3.11 <small><sup>27-08-2022</sup></small>

- [Docs] updated docs on isArrayOk (options.sortOrder)
- [Docs] updated changelog

# v1.3.10 <small><sup>25-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.9 <small><sup>24-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.8 <small><sup>24-08-2022</sup></small>

- [Update] critical tests & cleanups

# v1.3.7 <small><sup>19-08-2022</sup></small>

- [Fixed] lax properties behaving as required

# v1.3.6 <small><sup>18-08-2022</sup></small>

- [Added] possibility to use readonly as "lax"
- [Added] timestamp props to context

# v1.3.5 <small><sup>14-08-2022</sup></small>

- [Update] Performance optimizations

# v1.3.4 <small><sup>12-08-2022</sup></small>

- [Added] Support for single life cycle listeners
- [Docs] Updated docs

# v1.3.3 <small><sup>09-08-2022</sup></small>

- [Removed] removed \_isError from ApiError

# v1.3.2 <small><sup>06-08-2022</sup></small>

- [Added] more tests & typescript type annotations

# v1.3.1 <small><sup>05-08-2022</sup></small>

- [Added] extra checks to ensure proper validation during updates

# v1.3.0 <small><sup>05-08-2022</sup></small>

- [Added] added onChange property for both create & update events
- [Docs] updated docs for v1.2.5 & v1.2.6
- [Removed] support for onUpdate rule on side effect properties

# v1.2.6 <small><sup>04-08-2022</sup></small>

- [Update] added check for non-empty array for onCreate and onUpdate listeners

# v1.2.5 <small><sup>03-08-2022</sup></small>

- [Update] did parallellization of schema validation at creation & updates

# v1.2.4 <small><sup>21-08-2022</sup></small>

- [Clean up] Stopped typescript from transpiling tests
- [Docs] added link to validate.isCreditCardOk in readme.md

# v1.2.3 <small><sup>21-08-2022</sup></small>

- [Added] built-in validator validate.isCreditCardOk
- [Added] CI pipeline via github actions
- [Docs] Added docs for isCreditCardOk

# v1.2.2 <small><sup>21-07-2022</sup></small>

- [Docs] Updated link to changelog in README.md

# v1.2.1 <small><sup>21-07-2022</sup></small>

- [Fix] Side effect properties not being initialised

# v1.2.0 <small><sup>20-07-2022</sup></small>

- [Removed] Schema options.timestamp so only options.timestamps is supported

# v1.1.12 <small><sup>20-07-2022</sup></small>

- [Added] Possibility to pass a string/string[] as options when cloning an entity
- [Docs] Updated docs on structure of validator functions
- [Docs] Updated some errors in docs
- [Removed] Schema options.timestamp so only options.timestamps is supported

# v1.1.11 <small><sup>19-07-2022</sup></small>

- [Fix] error: Exported variable 'YourModel' has or is using name 'Model' from external module "clean-schema" but cannot be named

# v1.1.10 <small><sup>19-07-2022</sup></small>

- [Added] generics for typescript

# v1.1.9 <small><sup>19-07-2022</sup></small>

- [Fix] invalid properties crashing create & updates properly

# v1.1.8 <small><sup>19-07-2022</sup></small>

- [Fix] non readonly|required not being initialized
- [Fix] updates not working properly

# v1.1.7 <small><sup>18-07-2022</sup></small>

- [Added] Updated type definitions of onCreate/onUpdate listeners for typescript

# v1.1.6 <small><sup>18-07-2022</sup></small>

- [Added] Updated type definitions for typescript support

# v1.1.5 <small><sup>18-07-2022</sup></small>

- [Fix] Fixed issue with clean-schema requiring options parameter on Schema constructor

# v1.1.4 <small><sup>18-07-2022</sup></small>

- [Fix] Fixed issue with clean-schema requiring options parameter on SchemaCore constructor

# v1.1.3 <small><sup>18-07-2022</sup></small>

- [Deprecated] Changed schema options.timestamp to timestamps
- [Fix] Schema options still accepting timestamps with keys already on schema
- [Docs] Updated docs of how to use [validator.isEmailOk](./validate/isEmailOk.md)

# v1.1.2 <small><sup>17-07-2022</sup></small>

- [Docs] Added Changelog

# v1.1.1 <small><sup>17-07-2022</sup></small>

- [Added] Possibility to use custom regular expression with [validator.isEmailOk](./validate/isEmailOk.md)

- [Fix] Internal mechanism in charge of data manipulation which would cause bugs

# v1.1.0 <small><sup>17-07-2022</sup></small>

- [Docs] Imporved details of invalid schema error for onCreate & onUpdate methods

# v1.0.35 <small><sup>16-07-2022</sup></small>

- [Added] Built-in validator for emails [validator.isEmailOk](./validate/isEmailOk.md)
- [Docs] Improved details of invalid schema error

# v1.0.34 <small><sup>15-07-2022</sup></small>

- [Added] Possibility to override built in property names for timestamps
