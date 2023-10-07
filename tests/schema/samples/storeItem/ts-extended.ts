import { Schema } from '../../../../dist'
import { KeyOf } from '../../../../src/schema/types'
import { FieldError, IValidationError } from '../../../../src/schema/utils'
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

extended.create().then(({ data }) => {
  data?.age
})

type ErrorData<Keys> = { key: Keys; message: string }[]

class VError<Keys> extends Error implements IValidationError<ErrorData<Keys>> {
  constructor(public message: IValidationError<ErrorData<Keys>>['message']) {
    super(message)
  }
  add(
    field: PayloadKey,
    value?: string | string[] | FieldError | undefined
  ): this {
    throw new Error('Method not implemented.')
  }

  get data(): ErrorData<Keys> {
    return {} as ErrorData<Keys>
  }
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
  name: string
  stack?: string | undefined
}
