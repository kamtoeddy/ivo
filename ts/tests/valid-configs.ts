import { Schema } from '../src';

type PublicOnly = {
  dob: Date | string;
  name: string;
};

const publicOnly = new Schema<PublicOnly>({
  dob: { default: '' },
  name: { required: true, validator: () => true },
}).getModel();

publicOnly.create({ dob: '', name: '' }, null as any).then(({ data }) => {
  data?.dob;
  data?.name;
});

type PrivateOnly = {
  constants: string;
};
const outputOnly = new Schema<{}, PrivateOnly>({
  constants: { constant: true, value: '' },
}).getModel();

outputOnly.create({}, null as any).then(({ data }) => {
  data?.constants;
});

type PrivateAndPublicInput = {
  public: string;
};

type PrivateAndPublic = {
  constants: string;
  dependents: string;
  public: string;
};

const privatesAndPublic = new Schema<PrivateAndPublicInput, PrivateAndPublic>({
  constants: { constant: true, value: '' },
  dependents: {
    default: '',
    dependsOn: 'public',
    resolver: () => '',
  },
  public: { required: true, validator: () => true },
}).getModel();

privatesAndPublic.create({ public: '' }, null as any).then(({ data }) => {
  data?.constants;
  data?.dependents;
  data?.public;
});

type VirtualsAndPrivateInput = {
  virtual: string;
};

type VirtualsAndPrivate = {
  dependents: string;
};

const virtualsAndPrivate = new Schema<
  VirtualsAndPrivateInput,
  VirtualsAndPrivate
>({
  dependents: {
    default: '',
    dependsOn: 'virtual',
    resolver: () => '',
  },
  virtual: { virtual: true, validator: () => true },
}).getModel();

virtualsAndPrivate.create({ virtual: '' }, null as any).then(({ data }) => {
  data?.dependents;
});

type VirtualsPublicAndPrivateInput = {
  virtual: string;
};

type VirtualsPublicAndPrivate = {
  dependents: string;
};

const virtualsPublicAndPrivate = new Schema<
  VirtualsPublicAndPrivateInput,
  VirtualsPublicAndPrivate
>({
  dependents: {
    default: '',
    dependsOn: 'virtual',
    resolver: () => '',
  },
  virtual: { virtual: true, validator: () => true },
}).getModel();

virtualsPublicAndPrivate
  .create({ virtual: '' }, null as any)
  .then(({ data }) => {
    data?.dependents;
  });
