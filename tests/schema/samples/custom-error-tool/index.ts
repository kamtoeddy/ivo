import { KeyOf, Schema } from '../../../../dist';
import { VError } from './error-tool';

type Input = {
  name: string;
  dob: string;
};

type Output = {
  name: string;
  dob: string;
  age: string;
};

const schema = new Schema<Input, Output, {}, VError<KeyOf<Input>>>(
  {
    age: { default: '', dependsOn: 'dob', resolver: () => '' },
    dob: {
      default: '',
      validator: () => ({ valid: false, reason: 'Invalid date' })
    },
    name: { default: '', validator: () => false }
  },
  { errorTool: VError, errors: 'throw' }
);

const model = schema.getModel();

console.time('perf');

//   throw new VError('lol').throw();
model
  .create({ dob: new Date().toISOString(), name: 'lol' })
  .then(({ data, error }) => {
    error?.errors[0]?.field == 'dob';

    console.log(data ?? error);
  })
  .catch((err) => {
    console.log({ ...err });
  });

// const extended = schema
//   .extend<EInput, EOutput>({
//     lol: { default: '' },
//     dependent: { default: '', dependsOn: 'lol', resolver: () => '' },
//     v: { virtual: true, validator: () => true }
//   })
//   .getModel()

// extended.create().then(({ error }) => {
//   error?.errors[0].key == 'dob'
// })
