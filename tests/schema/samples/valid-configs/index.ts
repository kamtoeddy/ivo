import { Schema } from '../../../../dist'

type PublicOnly = {
  dob: Date | string
  name: string
}

const publicOnly = new Schema<PublicOnly>({
  dob: { default: '' },
  name: { required: true, validator: () => true }
}).getModel()

publicOnly.create({ dob: '', name: '' }).then(({ data }) => {
  data?.dob
  data?.name
})

type PrivateOnly = {
  constants: string
}
const outputOnly = new Schema<PrivateOnly, {}>({
  constants: { constant: true, value: '' }
}).getModel()

outputOnly.create({}).then(({ data }) => {
  data?.constants
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
