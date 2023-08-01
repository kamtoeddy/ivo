export {
  expectFailure,
  expectNoFailure,
  expectPromiseFailure,
  getValidSchema,
  makeFx,
  pauseFor,
  validator
}

const expectFailure = (fx: Function, message = 'Invalid Schema') => {
  expect(fx).toThrow(message)
}

const expectPromiseFailure = (fx: Function, message = 'Invalid Schema') => {
  expect(fx).rejects.toThrow(message)
}

const expectNoFailure = (fx: Function) => {
  expect(fx).not.toThrow()
}

const getValidSchema = (extraDefinition = {}, extraProps = {}) => ({
  propertyName1: { default: '', validator, ...extraDefinition },
  propertyName2: { default: '', validator },
  ...extraProps
})

const makeFx =
  (Schema: any) =>
  (definition: any = undefined, options: any = { timestamps: false }) =>
  () =>
    new Schema(definition, options)

const pauseFor = (ms = 5) =>
  new Promise((res) => setTimeout(() => res(true), ms))

const validator = () => ({ valid: true })
