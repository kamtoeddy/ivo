import { describe, it, expect } from 'vitest';

import { ERRORS } from '../../../dist';
import { expectFailure, expectNoFailure, getValidSchema } from '../_utils';

export const Test_SchemaOptionPostValidate = ({ fx }: any) => {
  describe('Schema.options.postValidate', () => {
    describe('config', () => {
      it("should allow 'postValidate' as valid config or array of valid configs", () => {
        const values = [
          { properties: ['propertyName1', 'propertyName2'], handler() {} },
          [{ properties: ['propertyName1', 'propertyName2'], handler() {} }]
        ];

        for (const postValidate of values) {
          const toPass = fx(getValidSchema(), { postValidate });

          expectNoFailure(toPass);

          toPass();
        }
      });

      describe('single config', () => {
        it("should reject 'postValidate' as invaild single config", () => {
          const invalidPostValidateSingleConfig = [
            'The "postValidate" option must be an object with keys "properties" and "handler" or an array of "PostValidateConfig"'
          ];

          const values = [
            -1,
            0,
            1,
            null,
            undefined,
            true,
            false,
            '',
            'invalid',
            {},
            { properties: [] },
            { handler: [] },
            () => {}
          ];

          for (const postValidate of values) {
            const toFail = fx(getValidSchema(), { postValidate });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining(
                    invalidPostValidateSingleConfig
                  )
                }
              });
            }
          }
        });

        it("should reject if 'properties' is not an array", () => {
          const values = [
            -1,
            0,
            1,
            null,
            undefined,
            true,
            false,
            '',
            'invalid',
            {},
            { properties: [] },
            { handler: [] },
            () => {},
            [],
            ['lol'],
            ['lol', 'lolol'],
            ['propertyName1', 'lolol'],
            ['propertyName1', 'propertyName1'],
            ['propertyName1', 'dependent']
          ];

          for (const properties of values) {
            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {}
                  }
                }
              ),
              {
                postValidate: { properties, handler() {} }
              }
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining([
                    '"properties" must be an array of at least 2 input properties of your schema'
                  ])
                }
              });
            }
          }
        });

        it("should reject if 'handler' is not a function", () => {
          const values = [
            -1,
            0,
            1,
            null,
            undefined,
            true,
            false,
            '',
            'invalid',
            {},
            []
          ];

          for (const handler of values) {
            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependent: {
                    default: '',
                    dependsOn: 'propertyName1',
                    resolver() {}
                  }
                }
              ),
              {
                postValidate: {
                  properties: ['propertyName1', 'propertyName2'],
                  handler
                }
              }
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ERRORS.INVALID_SCHEMA,
                payload: {
                  postValidate: expect.arrayContaining([
                    '"handler" must be a function'
                  ])
                }
              });
            }
          }
        });

        it('should reject if config has any extra property', () => {
          const toFail = fx(getValidSchema(), {
            postValidate: {
              properties: ['propertyName1', 'propertyName2'],
              handler() {},
              lol: true
            }
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ERRORS.INVALID_SCHEMA,
              payload: {
                postValidate: expect.arrayContaining([
                  'The "postValidate" option must be an object with keys "properties" and "handler" or an array of "PostValidateConfig"'
                ])
              }
            });
          }
        });
      });
    });
  });
};
