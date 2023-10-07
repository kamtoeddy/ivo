import { Summary, Schema } from '../../../../dist'
import { KeyOf } from '../../../../src/schema/types'
import { IValidationError } from '../../../../src/schema/utils'

type Input = {
  name: string
  dob: string
}

type Output = {
  name: string
  dob: string
  age: string
}

const schema = new Schema<Output, Input, {}, VError<KeyOf<Input>>>({
  age: { default: '', dependsOn: 'dob', resolver: () => '' },
  dob: { default: '' },
  name: { default: '' }
})

const model = schema.getModel()

// model.create().then(({ error }) => {
//   error[0].
// })

type EInput = {
  name: string
  dob: string
  lol: string
}

type EOutput = {
  name: string
  lol: string
  deps: string
  dob: string
  age: string
}

const extended = schema
  .extend<EOutput, EInput>({
    lol: { default: '' },
    deps: { default: '', dependsOn: 'lol', resolver: () => '' }
  })
  .getModel()

// extended.create().then(({ error }) => {
//   error[0].key=''
// })

type ErrorData<Keys> = { key: Keys; message: string }[]

class VError<Keys> extends Error implements IValidationError<ErrorData<Keys>> {
  constructor(public message: IValidationError<ErrorData<Keys>>['message']) {
    super(message)
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
