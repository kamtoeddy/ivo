import { KeyOf, Schema } from '../../../../dist';
import { VError } from './error-tool';
import { EInput, EOutput, Input, Output } from './types';

export { UserModel, UserModel_ErrorThrow, EUserModel, EUserModel_ErrorThrow };

const userSchema = new Schema<Input, Output, {}, VError<KeyOf<Input>>>(
  {
    firstName: {
      default: '',
      validator: (v) => ({
        valid: false,
        reason: 'Invalid first name',
        metadata: { hint: 'try harder 😜', valueProvided: v }
      })
    },
    fullName: {
      default: '',
      dependsOn: ['firstName', 'lastName'],
      resolver: ({ context: { firstName, lastName } }) =>
        `${firstName}-${lastName}`
    },
    lastName: { default: '', validator: () => false }
  },
  { ErrorTool: VError }
);

const UserModel = userSchema.getModel();
const UserModel_ErrorThrow = userSchema
  .extend({}, { errors: 'throw' })
  .getModel();

const EUserSchema = userSchema.extend<EInput, EOutput>(
  {
    full_name: {
      default: '',
      dependsOn: ['firstName', 'lastName'],
      resolver: ({ context: { firstName, lastName } }) =>
        `${firstName}-${lastName}`
    }
  },
  { remove: 'fullName' }
);

const EUserModel = EUserSchema.getModel();

const EUserModel_ErrorThrow = EUserSchema.extend(
  {},
  { errors: 'throw' }
).getModel();
