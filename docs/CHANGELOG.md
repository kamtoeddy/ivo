# Changelog

All notable changes to this project will be documented in this file.

# v1.3.1 <small><sup>26-04-2024</sup></small>

- Update type inference of isArrayOk, isNumberOk and isStringOk utilities
- Exported isOneOf utility

# v1.3.0 <small><sup>14-04-2024</sup></small>

- Split Context into ImmutableContext & MutableContext and Summary into ImmutableSummary & MutableSummary
- Add `__updateOptions__` method to MutableContext to help update CtxOptions from within an operation
- Fix issue with post-validation config not rejecting invalid properties
- Fix issue with ctx options not being reset on every operation

# v1.2.3 <small><sup>31-03-2024</sup></small>

- Fix issue with built-in validator casting non-nullable values to string

# v1.2.2 <small><sup>28-03-2024</sup></small>

- Improve type inference of schema definition

# v1.2.1 <small><sup>22-02-2024</sup></small>

- Fixed issue causing secondary validators not to respect `shouldInit` & `shouldUpdate` rules

# v1.2.0 <small><sup>22-02-2024</sup></small>

- Fixed issue with `DeletionContext` being exported as `DeleteContext`
- Cleaned up tests related to post-validation and secondary validations
- Updated docs on secondary validations

# v1.1.0 <small><sup>20-02-2024</sup></small>

- Added support for upto 2 validators per property (Primary & Secondary)

# v1.0.2 <small><sup>06-02-2024</sup></small>

Fixed bugs related to use with cjs

# v1.0.1 <small><sup>01-02-2024</sup></small>

first release

# v0.0.1 <small><sup>08-12-2023</sup></small>

package setup
