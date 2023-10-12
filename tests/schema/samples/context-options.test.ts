import { describe, expect, it } from 'vitest';

import {
  UserModel,
  UserModel_ErrorThrow,
  EUserModel,
  EUserModel_ErrorThrow
} from './custom-error-tool';

const contextOptions = { lang: 'en' };

describe('Context options', () => {
  describe('Base Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should respect ctx options with Model.clone', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await UserModel.clone(
          {
            firstName,
            lastName
          },
          { contextOptions }
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel.update(
          { firstName: 'John', fullName: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should respect ctx options with Model.clone', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await UserModel_ErrorThrow.clone(
          {
            firstName,
            lastName
          },
          { contextOptions }
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await UserModel_ErrorThrow.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await UserModel_ErrorThrow.update(
          { firstName: 'John', fullName: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });
  });

  describe('Extended Model', () => {
    describe('Schema.options.errors == silent', () => {
      it('should respect ctx options with Model.clone', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await EUserModel.clone(
          {
            firstName,
            lastName
          },
          { contextOptions }
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel.update(
          { firstName: 'John', full_name: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });

    describe('Schema.options.errors == throw', () => {
      it('should respect ctx options with Model.clone', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await EUserModel_ErrorThrow.clone(
          {
            firstName,
            lastName
          },
          { contextOptions }
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.create', async () => {
        const firstName = 'name',
          lastName = '';

        const { data, error } = await EUserModel_ErrorThrow.create(
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });

      it('should respect ctx options with Model.update', async () => {
        const firstName = 'name',
          lastName = 'lname';

        const { data, error } = await EUserModel_ErrorThrow.update(
          { firstName: 'John', full_name: '', lastName: 'lopa' },
          {
            firstName,
            lastName
          },
          contextOptions
        );

        expect(data).not.toBeNull();
        expect(error).toBeNull();
      });
    });
  });
});
