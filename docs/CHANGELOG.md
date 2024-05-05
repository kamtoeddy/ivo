# Changelog

All notable changes to this project will be documented in this file.

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
