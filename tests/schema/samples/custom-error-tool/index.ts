import { type KeyOf, Schema } from '../../../../dist';
import { VError } from './error-tool';
import type { Ctx_Options, EInput, EOutput, Input, Output } from './types';

export { EUserModel, UserModel };

const userSchema = new Schema<
  Input,
  Output,
  {},
  Ctx_Options,
  VError<KeyOf<Input>>
>(
  {
    firstName: {
      default: '',
      validator: (v, { options }) => {
        const { lang } = options;

        if (lang) return true;

        return {
          valid: false,
          reason: 'Invalid first name',
          metadata: { hint: 'try harder 😜', valueProvided: v },
        };
      },
    },
    fullName: {
      default: '',
      dependsOn: ['firstName', 'lastName'],
      resolver: ({ ctx: { firstName, lastName } }) =>
        `${firstName}-${lastName}`,
    },
    lastName: {
      default: '',
      validator: (_, { options }) => !!options.lang,
    },
  },
  { ErrorTool: VError },
);

const UserModel = userSchema.getModel();

const EUserSchema = userSchema.extend<EInput, EOutput>(
  {
    full_name: {
      default: '',
      dependsOn: ['firstName', 'lastName'],
      resolver: ({ ctx: { firstName, lastName } }) =>
        `${firstName} ${lastName}`,
    },
  },
  { remove: 'fullName' },
);

const EUserModel = EUserSchema.getModel();
