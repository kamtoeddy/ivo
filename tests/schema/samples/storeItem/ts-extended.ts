import { Schema } from '../../../../dist'
import { KeyOf } from '../../../../src/schema/types'
import {
  FieldError,
  IValidationError,
  IValidationErrorData
} from '../../../../src/schema/utils'
import { PayloadKey } from '../../../../src/utils'

type Input = {
  name: string
  dob: string
}

type Output = {
  name: string
  dob: string
  age: string
}

const schema = new Schema<Input, Output, {}, VError<KeyOf<Input>>>({
  age: { default: '', dependsOn: 'dob', resolver: () => '' },
  dob: { default: '' },
  name: { default: '' }
})

// const model = schema.getModel()

// model.create({}).then(({ error }) => {
//   error?.[0].key
// })

type EInput = {
  name: string
  dob: string
  lol: string
  v: string
}

type EOutput = {
  age: number
  dependent: string
  dob: string
  lol: string
  name: string
}

const extended = schema
  .extend<EInput, EOutput>({
    lol: { default: '' },
    dependent: { default: '', dependsOn: 'lol', resolver: () => '' },
    v: { virtual: true, validator: () => true }
  })
  .getModel()

extended.create().then(({ error }) => {
  error?.errors[0].key == 'dob'
})

type ErrorData<Keys> = { errors: { key: Keys; message: string }[] }

class VError<Keys> implements IValidationError<ErrorData<Keys>> {
  constructor(public message) {}
  get data(): IValidationErrorData<ErrorData<Keys>> {
    throw new Error('Method not implemented.')
  }

  add(
    field: PayloadKey,
    value?: string | string[] | FieldError | undefined
  ): this {
    throw new Error('Method not implemented.')
  }

  // get data() {
  //   return { message: this.message, errors: [] }
  // }
  get isLoaded(): boolean {
    throw new Error('Method not implemented.')
  }
  get fields(): any {
    throw new Error('Method not implemented.')
  }
  setMessage(
    message: 'INVALID_DATA' | 'NOTHING_TO_UPDATE' | 'VALIDATION_ERROR'
  ): this {
    throw new Error('Method not implemented.')
  }
  throw(): never {
    throw new Error('Method not implemented.')
  }
}
