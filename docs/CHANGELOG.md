# Changelog

All notable changes to this project will be documented in this file.

# 1.2.3 / 21-08-2022

- [Added] built-in validator validate.isCreditCardOk
- [Added] CI pipeline via github actions
- [Docs] Added docs for isCreditCardOk

# 1.2.2 / 21-07-2022

- [Docs] Updated link to changelog in README.md

# 1.2.1 / 21-07-2022

- [Fix] Side effect properties not being initialised

# 1.2.0 / 20-07-2022

- [Removed] Schema options.timestamp so only options.timestamps is supported

# 1.1.12 / 20-07-2022

- [Added] Possibility to pass a string/string[] as options when cloning an entity
- [Docs] Updated docs on structure of validator functions
- [Docs] Updated some errors in docs
- [Removed] Schema options.timestamp so only options.timestamps is supported

# 1.1.11 / 19-07-2022

- [Fix] error: Exported variable 'YourModel' has or is using name 'Model' from external module "clean-schema" but cannot be named

# 1.1.10 / 19-07-2022

- [Added] generics for typescript

# 1.1.9 / 19-07-2022

- [Fix] invalid properties crashing create & updates properly

# 1.1.8 / 19-07-2022

- [Fix] non readonly|required not being initialized
- [Fix] updates not working properly

# 1.1.7 / 18-07-2022

- [Added] Updated type definitions of onCreate/onUpdate handlers for typescript

# 1.1.6 / 18-07-2022

- [Added] Updated type definitions for typescript support

# 1.1.5 / 18-07-2022

- [Fix] Fixed issue with clean-schema requiring options parameter on Schema constructor

# 1.1.4 / 18-07-2022

- [Fix] Fixed issue with clean-schema requiring options parameter on SchemaCore constructor

# 1.1.3 / 18-07-2022

- [Deprecated] Changed schema options.timestamp to timestamps
- [Fix] Schema options still accepting timestamps with keys already on schema
- [Docs] Updated docs of how to use [validator.isEmailOk](./validate/isEmailOk.md)

# 1.1.2 / 17-07-2022

- [Docs] Added Changelog

# 1.1.1 / 17-07-2022

- [Added] Possibility to use custom regular expression with [validator.isEmailOk](./validate/isEmailOk.md)

- [Fix] Internal mechanism in charge of data manipulation which would cause bugs

# 1.1.0 / 17-07-2022

- [Docs] Imporved details of invalid schema error for onCreate & onUpdate methods

# 1.0.35 / 16-07-2022

- [Added] Built-in validator for emails [validator.isEmailOk](./validate/isEmailOk.md)
- [Docs] Improved details of invalid schema error

# 1.0.34 / 15-07-2022

- [Added] Possibility to override built in property names for timestamps
