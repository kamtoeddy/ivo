import { Summary, Schema } from '../../../../../dist'

type IOSame = {
  dob: Date | string
  name: string
}

const sameIO = new Schema<IOSame>({
  dob: { default: '' },
  name: { required: true, validator: () => true }
}).getModel()

sameIO.create({ dob: '', name: '' }).then(({ data }) => {
  data?.dob
  data?.name
})

type PrivateOnly = {
  constants: string
  dependents: string
}
const outputOnly = new Schema<PrivateOnly, {}>({
  constants: { constant: true, value: '' },
  dependents: {
    default: '',
    dependent: true,
    dependsOn: 'constants',
    resolver: () => ''
  }
}).getModel()

outputOnly.create({ yoo: '' }).then(({ data }) => {
  data?.constants
  data?.dependents
})

type PrivateAndPublicInput = {
  public: string
}

type PrivateAndPublic = {
  constants: string
  dependents: string
  public: string
}

const privatesAndPublic = new Schema<PrivateAndPublic, PrivateAndPublicInput>({
  constants: { constant: true, value: '' },
  dependents: {
    default: '',
    dependent: true,
    dependsOn: 'public',
    resolver: () => ''
  },
  public: { required: true, validator: () => true }
}).getModel()

privatesAndPublic.create({ public: '' }).then(({ data }) => {
  data?.constants
  data?.dependents
  data?.public
})

type VirtualsAndPrivateInput = {
  virtual: string
}

type VirtualsAndPrivate = {
  dependents: string
}

const virtualsAndPrivate = new Schema<
  VirtualsAndPrivate,
  VirtualsAndPrivateInput
>({
  dependents: {
    default: '',
    dependent: true,
    dependsOn: 'virtual',
    resolver: () => ''
  },
  virtual: { virtual: true, validator: () => true }
}).getModel()

virtualsAndPrivate.create({ virtual: '' }).then(({ data }) => {
  data?.dependents
})

type VirtualsPublicAndPrivateInput = {
  virtual: string
}

type VirtualsPublicAndPrivate = {
  dependents: string
}

const virtualsPublicAndPrivate = new Schema<
  VirtualsPublicAndPrivate,
  VirtualsPublicAndPrivateInput
>({
  dependents: {
    default: '',
    dependent: true,
    dependsOn: 'virtual',
    resolver: () => ''
  },
  virtual: { virtual: true, validator: () => true }
}).getModel()

virtualsPublicAndPrivate.create({ virtual: '' }).then(({ data }) => {
  data?.dependents
})
