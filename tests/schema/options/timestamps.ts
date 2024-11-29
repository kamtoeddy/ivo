import { beforeEach, describe, expect, it } from 'bun:test';

import { expectFailure, expectNoFailure, getValidSchema } from '../_utils';

export const Test_SchemaTimestampOption = ({ Schema, fx }: any) => {
  describe('Schema.options.timestamps', () => {
    describe('valid', () => {
      it('should allow true | false', () => {
        const values = [false, true];

        for (const timestamps of values) {
          const toPass = fx(getValidSchema(), { timestamps });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it('should allow updated as object', () => {
        const values = [
          { key: 'updatedAt' },
          { nullable: true },
          { nullable: false },
          { key: 'updatedAt', nullable: false },
          { key: 'updatedAt', nullable: true },
        ];

        for (const updatedAt of values) {
          const toPass = fx(getValidSchema(), { timestamps: { updatedAt } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      describe('behaviour', () => {
        const inputValue = {
          propertyName1: 'value1',
          propertyName2: 'value2',
        };

        let onSuccessValues: any = {};

        beforeEach(() => {
          onSuccessValues = {};
        });

        describe('timestamps(true)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { createdAt, updatedAt } }: any) => {
              onSuccessValues.createdAt = createdAt;
              onSuccessValues.updatedAt = updatedAt;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: true,
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate createdAt & updatedAt at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).toHaveProperty('createdAt');
            expect(entity).toHaveProperty('updatedAt');

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeNull();
          });

          it('should populate updatedAt during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).toHaveProperty('updatedAt');

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeNull();
          });
        });

        describe('timestamps(createdAt:c_At)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { c_At, updatedAt } }: any) => {
              onSuccessValues.c_At = c_At;
              onSuccessValues.updatedAt = updatedAt;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { createdAt: 'c_At' },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate c_At & updatedAt at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty('createdAt');
            expect(entity).toHaveProperty('c_At');
            expect(entity).toHaveProperty('updatedAt');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeNull();
          });

          it('should populate updatedAt during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('c_At');
            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).toHaveProperty('updatedAt');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeNull();
          });
        });

        describe('timestamps(updatedAt:u_At)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { createdAt, u_At } }: any) => {
              onSuccessValues.createdAt = createdAt;
              onSuccessValues.u_At = u_At;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { updatedAt: 'u_At' },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate createdAt & u_At at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty('updatedAt');
            expect(entity).toHaveProperty('createdAt');
            expect(entity).toHaveProperty('u_At');

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });

          it('should populate u_At during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).not.toHaveProperty('updatedAt');
            expect(updates).toHaveProperty('u_At');

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });
        });

        describe('timestamps(createdAt:c_At, updatedAt:u_At)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { c_At, u_At } }: any) => {
              onSuccessValues.c_At = c_At;
              onSuccessValues.u_At = u_At;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { createdAt: 'c_At', updatedAt: 'u_At' },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate c_At & u_At at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty('createdAt');
            expect(entity).not.toHaveProperty('updatedAt');
            expect(entity).toHaveProperty('c_At');
            expect(entity).toHaveProperty('u_At');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });

          it('should populate u_At during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).not.toHaveProperty('updatedAt');
            expect(updates).not.toHaveProperty('c_At');
            expect(updates).toHaveProperty('u_At');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });
        });

        describe('timestamps(createdAt:c_At, updatedAt:false)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { c_At, updatedAt } }: any) => {
              onSuccessValues.c_At = c_At;
              onSuccessValues.updatedAt = updatedAt;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { createdAt: 'c_At', updatedAt: false },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate only c_At at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty('createdAt');
            expect(entity).not.toHaveProperty('updatedAt');
            expect(Object.keys(entity).length).toBe(3);

            expect(entity).toHaveProperty('c_At');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeUndefined();
          });

          it('should not populate updatedAt during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).not.toHaveProperty('updatedAt');
            expect(Object.keys(updates).length).toBe(1);

            expect(updates).not.toHaveProperty('c_At');

            expect(onSuccessValues.c_At).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeUndefined();
          });
        });

        describe('timestamps(updatedAt:false)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { createdAt, updatedAt } }: any) => {
              onSuccessValues.createdAt = createdAt;
              onSuccessValues.updatedAt = updatedAt;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { updatedAt: false },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate only createdAt at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).toHaveProperty('createdAt');
            expect(entity).not.toHaveProperty('updatedAt');
            expect(Object.keys(entity).length).toBe(3);

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeUndefined();
          });

          it('should not populate updatedAt during updates', async () => {
            const { data: updates, handleSuccess } = await Model.update(
              entity,
              { propertyName2: 20 },
            );

            await handleSuccess();

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('updatedAt');
            expect(Object.keys(updates).length).toBe(1);

            expect(onSuccessValues.createdAt).toBeDefined();
            expect(onSuccessValues.updatedAt).toBeUndefined();
          });
        });

        describe('timestamps(createdAt:false, updatedAt:u_At)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { createdAt, u_At } }: any) => {
              onSuccessValues.createdAt = createdAt;
              onSuccessValues.u_At = u_At;
            };

            Model = new Schema(getValidSchema({ onSuccess }), {
              timestamps: { createdAt: false, updatedAt: 'u_At' },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate only u_At at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).toHaveProperty('u_At');
            expect(entity).not.toHaveProperty('createdAt');
            expect(entity).not.toHaveProperty('updatedAt');
            expect(Object.keys(entity).length).toBe(3);

            expect(onSuccessValues.createdAt).toBeUndefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });

          it('should populate only u_At during updates', async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).not.toHaveProperty('updatedAt');
            expect(Object.keys(updates).length).toBe(2);

            expect(updates).toHaveProperty('u_At');

            expect(onSuccessValues.createdAt).toBeUndefined();
            expect(onSuccessValues.u_At).toBeDefined();
          });
        });

        describe('timestamps(createdAt:false)', () => {
          let Model: any, entity: any;

          beforeEach(async () => {
            const onSuccess = ({ context: { createdAt, updatedAt } }: any) => {
              onSuccessValues.createdAt = createdAt;
              onSuccessValues.updatedAt = updatedAt;
            };

            Model = new Schema(getValidSchema(), {
              onSuccess,
              timestamps: { createdAt: false },
            }).getModel();

            const res = await Model.create(inputValue);

            entity = res.data;

            await res.handleSuccess();
          });

          it('should populate only updatedAt at creation', () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty('createdAt');
            expect(entity).toHaveProperty('updatedAt');
            expect(Object.keys(entity).length).toBe(3);

            expect(onSuccessValues.createdAt).toBeUndefined();
            expect(onSuccessValues.updatedAt).toBeNull();
          });

          it('should populate only updatedAt during updates', async () => {
            const { data: updates, handleSuccess } = await Model.update(
              entity,
              { propertyName2: 20 },
            );

            await handleSuccess();

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty('createdAt');
            expect(updates).toHaveProperty('updatedAt');
            expect(Object.keys(updates).length).toBe(2);

            expect(onSuccessValues.createdAt).toBeUndefined();
            expect(onSuccessValues.updatedAt).not.toBeNull();
          });
        });

        describe('timestamps(updatedAt:object)', () => {
          describe('updatedAt(nullable:true)', () => {
            let Model: any, entity: any;

            beforeEach(async () => {
              const onSuccess = ({
                context: { createdAt, updatedAt },
              }: any) => {
                onSuccessValues.createdAt = createdAt;
                onSuccessValues.updatedAt = updatedAt;
              };

              Model = new Schema(getValidSchema(), {
                onSuccess,
                timestamps: { updatedAt: { nullable: true } },
              }).getModel();

              const res = await Model.create(inputValue);

              entity = res.data;

              await res.handleSuccess();
            });

            it('should populate createdAt & updatedAt at creation', () => {
              expect(entity).toMatchObject(inputValue);

              expect(entity).toHaveProperty('createdAt');
              expect(entity).toHaveProperty('updatedAt');

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues.updatedAt).toBeNull();
            });

            it('should populate updatedAt during updates', async () => {
              const { data: updates } = await Model.update(entity, {
                propertyName2: 20,
              });

              expect(updates).toMatchObject({ propertyName2: 20 });

              expect(updates).not.toHaveProperty('createdAt');
              expect(updates).toHaveProperty('updatedAt');

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues.updatedAt).toBeNull();
            });
          });

          describe('updatedAt(nullable:false)', () => {
            let Model: any, entity: any;

            beforeEach(async () => {
              const onSuccess = ({
                context: { createdAt, updatedAt },
              }: any) => {
                onSuccessValues.createdAt = createdAt;
                onSuccessValues.updatedAt = updatedAt;
              };

              Model = new Schema(getValidSchema(), {
                onSuccess,
                timestamps: { updatedAt: { nullable: false } },
              }).getModel();

              const res = await Model.create(inputValue);

              entity = res.data;

              await res.handleSuccess();
            });

            it('should populate createdAt & updatedAt at creation', () => {
              expect(entity).toMatchObject(inputValue);

              expect(entity).toHaveProperty('createdAt');
              expect(entity).toHaveProperty('updatedAt');

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues.updatedAt).not.toBeNull();
            });

            it('should populate updatedAt during updates', async () => {
              const { data: updates } = await Model.update(entity, {
                propertyName2: 20,
              });

              expect(updates).toMatchObject({ propertyName2: 20 });

              expect(updates).not.toHaveProperty('createdAt');
              expect(updates).toHaveProperty('updatedAt');

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues.updatedAt).not.toBeNull();
            });
          });

          describe('updatedAt(key:uAt, nullable:true)', () => {
            let Model: any, entity: any;
            const updatedAtKey = 'uAt';

            beforeEach(async () => {
              const onSuccess = ({ context }: any) => {
                onSuccessValues.createdAt = context.createdAt;
                onSuccessValues[updatedAtKey] = context[updatedAtKey];
              };

              Model = new Schema(getValidSchema(), {
                onSuccess,
                timestamps: {
                  updatedAt: { key: updatedAtKey, nullable: true },
                },
              }).getModel();

              const res = await Model.create(inputValue);

              entity = res.data;

              await res.handleSuccess();
            });

            it('should populate createdAt & updatedAt at creation', () => {
              expect(entity).toMatchObject(inputValue);

              expect(entity).toHaveProperty('createdAt');
              expect(entity).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).toBeNull();
            });

            it('should populate updatedAt during updates', async () => {
              const { data: updates, handleSuccess } = await Model.update(
                entity,
                { propertyName2: 20 },
              );

              await handleSuccess();

              expect(updates).toMatchObject({ propertyName2: 20 });

              expect(updates).not.toHaveProperty('createdAt');
              expect(updates).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).not.toBeNull();
            });
          });

          describe('updatedAt(key:uAt, nullable:undefined)', () => {
            let Model: any, entity: any;
            const updatedAtKey = 'uAt';

            beforeEach(async () => {
              const onSuccess = ({ context }: any) => {
                onSuccessValues.createdAt = context.createdAt;
                onSuccessValues[updatedAtKey] = context[updatedAtKey];
              };

              Model = new Schema(getValidSchema(), {
                onSuccess,
                timestamps: {
                  updatedAt: { key: updatedAtKey },
                },
              }).getModel();

              const res = await Model.create(inputValue);

              entity = res.data;

              await res.handleSuccess();
            });

            it('should populate createdAt & updatedAt at creation', () => {
              expect(entity).toMatchObject(inputValue);

              expect(entity).toHaveProperty('createdAt');
              expect(entity).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).toBeNull();
            });

            it('should populate updatedAt during updates', async () => {
              const { data: updates, handleSuccess } = await Model.update(
                entity,
                { propertyName2: 20 },
              );

              await handleSuccess();

              expect(updates).toMatchObject({ propertyName2: 20 });

              expect(updates).not.toHaveProperty('createdAt');
              expect(updates).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).not.toBeNull();
            });
          });

          describe('updatedAt(key:uAt, nullable:false)', () => {
            let Model: any, entity: any;
            const updatedAtKey = 'uAt';

            beforeEach(async () => {
              const onSuccess = ({ context }: any) => {
                onSuccessValues.createdAt = context.createdAt;
                onSuccessValues[updatedAtKey] = context[updatedAtKey];
              };

              Model = new Schema(getValidSchema(), {
                onSuccess,
                timestamps: {
                  updatedAt: { key: updatedAtKey, nullable: false },
                },
              }).getModel();

              const res = await Model.create(inputValue);

              entity = res.data;

              await res.handleSuccess();
            });

            it('should populate createdAt & updatedAt at creation', () => {
              expect(entity).toMatchObject(inputValue);

              expect(entity).toHaveProperty('createdAt');
              expect(entity).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).not.toBeNull();
            });

            it('should populate updatedAt during updates', async () => {
              const { data: updates, handleSuccess } = await Model.update(
                entity,
                { propertyName2: 20 },
              );

              await handleSuccess();

              expect(updates).toMatchObject({ propertyName2: 20 });

              expect(updates).not.toHaveProperty('createdAt');
              expect(updates).toHaveProperty(updatedAtKey);

              expect(onSuccessValues.createdAt).toBeDefined();
              expect(onSuccessValues[updatedAtKey]).not.toBeNull();
            });
          });
        });
      });
    });

    describe('invalid', () => {
      it('should reject non boolean & non objects', () => {
        const values = [null, [], 1, '2asf'];

        for (const timestamps of values) {
          const toFail = fx(getValidSchema(), { timestamps });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                timestamps: expect.arrayContaining([
                  "should be 'boolean' or 'non null object'",
                ]),
              }),
            );
          }
        }
      });

      it('should reject empty object', () => {
        const toFail = fx(getValidSchema(), { timestamps: {} });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              timestamps: expect.arrayContaining(['cannot be an empty object']),
            }),
          );
        }
      });

      it('should reject custom name found on schema', () => {
        const values = [
          'dependentProp',
          'propertyName1',
          'propertyName2',
          'virtualProp',
        ];
        const timestampKeys = ['createdAt', 'updatedAt'];

        for (const key of timestampKeys) {
          for (const value of values) {
            const toFail = fx(
              getValidSchema(
                {},
                {
                  dependentProp: {
                    default: '',
                    dependsOn: 'virtualProp',
                    resolver: () => '',
                  },
                  virtualProp: { virtual: true, validator: () => true },
                },
              ),
              {
                timestamps: { [key]: value },
              },
            );

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  timestamps: expect.arrayContaining([
                    `'${value}' already belongs to your schema`,
                  ]),
                }),
              );
            }
          }
        }
      });

      it('should reject if custom timestamp names are the same', () => {
        const toFail = fx(getValidSchema(), {
          timestamps: { createdAt: 'c_At', updatedAt: 'c_At' },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              timestamps: expect.arrayContaining([
                'createdAt & updatedAt cannot be same',
              ]),
            }),
          );
        }
      });

      it("should reject empty strings for custom 'createdAt'", () => {
        const toFail = fx(getValidSchema(), { timestamps: { createdAt: '' } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              timestamps: expect.arrayContaining([
                "'createdAt' cannot be an empty string",
              ]),
            }),
          );
        }
      });

      it("should reject empty strings for custom 'updatedAt'", () => {
        const toFail = fx(getValidSchema(), { timestamps: { updatedAt: '' } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              timestamps: expect.arrayContaining([
                "'updatedAt' cannot be an empty string",
              ]),
            }),
          );
        }
      });

      it("should reject invalid 'updatedAt' objects", () => {
        const createdAt = 'createdAt';

        const values = [
          [{ key: createdAt }, 'createdAt & updatedAt cannot be same'],
          ...[{}, { lol: true }].map((v) => [
            v,
            "'updatedAt' can only accept properties 'key' and 'nullable'",
          ]),
          ...['', ' '].map((key) => [
            { key },
            "'updatedAt.key' must be a valid string",
          ]),
          ...[
            -1,
            0,
            1,
            null,
            '',
            ' ',
            'lol',
            'true',
            'false',
            undefined,
            {},
            [],
            () => {},
          ].map((nullable) => [
            { nullable },
            "'updatedAt.nullable' must be a boolean",
          ]),
          ...['constant', 'dependent', 'lax', 'propertyName1', 'virtual'].map(
            (key) => [{ key }, `'${key}' already belongs to your schema`],
          ),
        ] as const;

        for (const [updatedAt, error] of values) {
          const toFail = fx(
            getValidSchema(
              {},
              {
                constant: { constant: true, value: () => true },
                lax: { default: '' },
                dependent: {
                  default: '',
                  dependsOn: 'virtual',
                  resolver: () => true,
                },
                virtual: { virtual: true, validator: () => true },
              },
            ),
            { timestamps: { createdAt, updatedAt } },
          );

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                timestamps: expect.arrayContaining([error]),
              }),
            );
          }
        }
      });
    });
  });
};
