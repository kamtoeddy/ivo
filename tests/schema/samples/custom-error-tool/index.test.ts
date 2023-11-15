import { describe, expect, it } from 'vitest';

import { VALIDATION_ERRORS } from '../../../../dist';

import {
  UserModel,
  UserModel_ErrorThrow,
  EUserModel,
  EUserModel_ErrorThrow
} from '.';

describe('Custom ErrorTool', () => {
  describe('Base Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should return custom errors with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.create({
          firstName,
          lastName
        });

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.VALIDATION_ERROR,
          errors: expect.arrayContaining([
            expect.objectContaining({
              field: 'firstName',
              value: firstName,
              messages: expect.arrayContaining(['Invalid first name']),
              metadata: { hint: 'try harder 😜', valueProvided: firstName }
            }),
            expect.objectContaining({
              field: 'lastName',
              value: lastName,
              messages: expect.arrayContaining(['validation failed'])
            })
          ])
        });
      });

      it('should return custom errors with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.update(
          { firstName: 'John', fullName: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          }
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.VALIDATION_ERROR,
          errors: expect.arrayContaining([
            expect.objectContaining({
              field: 'firstName',
              value: firstName,
              messages: expect.arrayContaining(['Invalid first name']),
              metadata: { hint: 'try harder 😜', valueProvided: firstName }
            }),
            expect.objectContaining({
              field: 'lastName',
              value: lastName,
              messages: expect.arrayContaining(['validation failed'])
            })
          ])
        });
      });

      it('should return nothing to update with Model.update when nessecary', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await UserModel.update(
          { firstName, fullName: `${firstName} ${lastName}`, lastName },
          { firstName, lastName }
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.NOTHING_TO_UPDATE,
          errors: []
        });
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should return custom errors with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        try {
          await UserModel_ErrorThrow.create({
            firstName,
            lastName
          });
        } catch (error) {
          expect(error.message).toBeDefined();

          expect(error).toMatchObject({
            errors: expect.arrayContaining([
              expect.objectContaining({
                field: 'firstName',
                value: firstName,
                messages: expect.arrayContaining(['Invalid first name']),
                metadata: { hint: 'try harder 😜', valueProvided: firstName }
              }),
              expect.objectContaining({
                field: 'lastName',
                value: lastName,
                messages: expect.arrayContaining(['validation failed'])
              })
            ])
          });
        }
      });

      it('should return custom errors with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        try {
          await UserModel_ErrorThrow.update(
            { firstName: 'John', fullName: '', lastName: 'lopa' },
            {
              firstName,
              lastName
            }
          );
        } catch (error) {
          expect(error.message).toBeDefined();

          expect(error).toMatchObject({
            errors: expect.arrayContaining([
              expect.objectContaining({
                field: 'firstName',
                value: firstName,
                messages: expect.arrayContaining(['Invalid first name']),
                metadata: { hint: 'try harder 😜', valueProvided: firstName }
              }),
              expect.objectContaining({
                field: 'lastName',
                value: lastName,
                messages: expect.arrayContaining(['validation failed'])
              })
            ])
          });
        }

        try {
          await UserModel_ErrorThrow.update(
            { firstName, fullName: '', lastName },
            { firstName, lastName }
          );
        } catch (error) {
          expect(error.message).toBeDefined();
          expect(error).toMatchObject({ errors: [] });
        }
      });
    });
  });

  describe('Extended Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should return custom errors with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.create({
          firstName,
          lastName
        });

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.VALIDATION_ERROR,
          errors: expect.arrayContaining([
            expect.objectContaining({
              field: 'firstName',
              value: firstName,
              messages: expect.arrayContaining(['Invalid first name']),
              metadata: { hint: 'try harder 😜', valueProvided: firstName }
            }),
            expect.objectContaining({
              field: 'lastName',
              value: lastName,
              messages: expect.arrayContaining(['validation failed'])
            })
          ])
        });
      });

      it('should return custom errors with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.update(
          { firstName: 'John', full_name: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          }
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.VALIDATION_ERROR,
          errors: expect.arrayContaining([
            expect.objectContaining({
              field: 'firstName',
              value: firstName,
              messages: expect.arrayContaining(['Invalid first name']),
              metadata: { hint: 'try harder 😜', valueProvided: firstName }
            }),
            expect.objectContaining({
              field: 'lastName',
              value: lastName,
              messages: expect.arrayContaining(['validation failed'])
            })
          ])
        });
      });

      it('should return nothing to update with Model.update when nessecary', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await EUserModel.update(
          { firstName, full_name: `${firstName} ${lastName}`, lastName },
          { firstName, lastName }
        );

        expect(data).toBeNull();
        expect(error).toMatchObject({
          message: VALIDATION_ERRORS.NOTHING_TO_UPDATE,
          errors: []
        });
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should return custom errors with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        try {
          await EUserModel_ErrorThrow.create({
            firstName,
            lastName
          });
        } catch (error) {
          expect(error.message).toBeDefined();

          expect(error).toMatchObject({
            errors: expect.arrayContaining([
              expect.objectContaining({
                field: 'firstName',
                value: firstName,
                messages: expect.arrayContaining(['Invalid first name']),
                metadata: { hint: 'try harder 😜', valueProvided: firstName }
              }),
              expect.objectContaining({
                field: 'lastName',
                value: lastName,
                messages: expect.arrayContaining(['validation failed'])
              })
            ])
          });
        }
      });

      it('should return custom errors with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        try {
          await EUserModel_ErrorThrow.update(
            { firstName: 'John', full_name: '', lastName: 'lopa' },
            {
              firstName,
              lastName
            }
          );
        } catch (error) {
          expect(error.message).toBeDefined();

          expect(error).toMatchObject({
            errors: expect.arrayContaining([
              expect.objectContaining({
                field: 'firstName',
                value: firstName,
                messages: expect.arrayContaining(['Invalid first name']),
                metadata: { hint: 'try harder 😜', valueProvided: firstName }
              }),
              expect.objectContaining({
                field: 'lastName',
                value: lastName,
                messages: expect.arrayContaining(['validation failed'])
              })
            ])
          });
        }

        try {
          await UserModel_ErrorThrow.update(
            { firstName, fullName: '', lastName },
            { firstName, lastName }
          );
        } catch (error) {
          expect(error.message).toBeDefined();
          expect(error).toMatchObject({ errors: [] });
        }
      });
    });
  });
});
