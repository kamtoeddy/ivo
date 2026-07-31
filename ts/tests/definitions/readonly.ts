import { beforeAll, describe, expect, it } from 'bun:test';

import { createFieldBuilder } from '../../src/schema/fields';
import { expectFailure, expectNoFailure, validator } from '../_utils';

const field = createFieldBuilder<any, any>();

/**
 * Current schema-core only supports `readonly: true` (the old `readonly: 'lax'`
 * variant no longer exists). Its interaction rules, mirroring
 * `rs/tests/fields/required/ignore.rs::should_respect_the_readonly_rule` and
 * `rs/tests/fields/dependents.rs`'s readonly-freeze tests:
 *  - lax + readonly: locked once its value diverges from the static default.
 *  - dependent + readonly: resolver keeps running while value == default,
 *    freezes permanently once it has diverged.
 *  - required + readonly: now a VALID combination (unlike the pre-refactor
 *    rule which rejected it) — creation is normal, every subsequent update is
 *    unconditionally rejected as a no-op.
 */
export const Test_ReadonlyProperties = ({ Schema, fx }: any) => {
  describe('readonly', () => {
    describe('valid', () => {
      it('should allow readonly(true) + dependent + default', () => {
        const toPass = fx({
          dependentProp: field
            .dependent('dependentProp')
            .default('value')
            .dependsOn('prop')
            .resolve(() => 1)
            .readonly(),
          prop: field.lax('prop').default(''),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow readonly(true) + requiredBy', () => {
        const toPass = fx({
          propertyName: field
            .lax('propertyName')
            .default('')
            .validate(validator)
            .readonly()
            .required(() => true),
        });

        expectNoFailure(toPass);

        toPass();
      });

      it('should allow readonly(true) + strictly required', () => {
        const toPass = fx({
          propertyName: field
            .required('propertyName')
            .validate(validator)
            .readonly(),
        });

        expectNoFailure(toPass);

        toPass();
      });

      describe('behaviour', () => {
        let Model: any;

        beforeAll(() => {
          Model = new Schema({
            age: field.lax('age').default(null).readonly(),
            name: field.lax('name').default('Default Name'),
          }).getModel();
        });

        it('should not modify readonly props that have changed via life cycle listeners at creation', async () => {
          const { data } = await Model.create({ age: 25 });

          expect(data).toMatchObject({ age: 25, name: 'Default Name' });
        });

        it('should not modify readonly props that have changed via life cycle listeners during updates', async () => {
          const { data } = await Model.update(
            { age: null, name: 'Default Name' },
            { age: 25, name: 'YoYo' },
          );

          expect(data).toMatchObject({ age: 25, name: 'YoYo' });
        });

        it('should still accept updates while the readonly value still equals its default', async () => {
          const { data, error } = await Model.update(
            { age: null, name: 'Default Name' },
            { age: 30 },
          );

          expect(error).toBeNull();
          expect(data).toEqual({ age: 30 });
        });

        it('should permanently lock a readonly lax field once its value has diverged from the default', async () => {
          const { data, error } = await Model.update(
            { age: 30, name: 'Default Name' },
            { age: 40 },
          );

          // nothing-to-update sentinel: no error, but no data either
          expect(error).toBeNull();
          expect(data).toBeNull();
        });
      });

      describe('behaviour with readonly + strictly required', () => {
        let Book: any;
        const book = { title: 'A Book' };

        beforeAll(() => {
          Book = new Schema({
            title: field.required('title').validate(validator).readonly(),
          }).getModel();
        });

        it('should create normally, requiring the field once', async () => {
          const { data, error } = await Book.create({ title: 'A Book' });

          expect(error).toBeNull();
          expect(data).toEqual(book);
        });

        it('should permanently reject every subsequent update, regardless of value', async () => {
          const { data, error } = await Book.update(book, {
            title: 'A different title',
          });

          expect(error).toBeNull();
          expect(data).toBeNull();
        });
      });
    });

    describe('invalid', () => {
      it('should reject readonly !== true', () => {
        const values = [1, '', null, undefined, false, 'lax'];

        for (const readonly of values) {
          const toFail = fx({ propertyName: { default: '', readonly } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Readonly properties must have readonly as 'true'",
                ]),
              }),
            );
          }
        }
      });

      it('should reject readonly(true) + dependent & no default', () => {
        const toFail = fx({
          dependentProp: {
            dependsOn: 'prop',
            resolver: () => 1,
            readonly: true,
          },
          prop: { default: '' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                'Dependent properties must have a default value',
              ]),
            }),
          );
        }
      });
    });
  });
};
