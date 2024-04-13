import { expect } from 'bun:test';
import { ERRORS, isEqual, ObjectType } from '../../dist';

export {
  expectFailure,
  expectNoFailure,
  findBy,
  getSubObject,
  getValidSchema,
  makeFx,
  validator,
};

function expectFailure(fx: Function, message: string = ERRORS.INVALID_SCHEMA) {
  expect(fx).toThrow(message);
}

function expectNoFailure(fx: Function) {
  expect(fx).not.toThrow();
}

function findBy<T>(list: T[] = [], determinant: any): T | undefined {
  return list.find((dt) => {
    const sub = getSubObject(dt as ObjectType, determinant);

    return isEqual(determinant, sub);
  });
}

function getDeepValue(data: ObjectType, key: string): any {
  return key.split('.').reduce((prev, next) => prev?.[next], data);
}

function getSubObject(obj: ObjectType, sampleSub: ObjectType) {
  const _obj: ObjectType = {},
    keys = Object.keys(sampleSub);

  keys.forEach((key) => (_obj[key] = getDeepValue(obj, key)));

  return _obj;
}

function getValidSchema(extraDefinition = {}, extraProps = {}) {
  return {
    propertyName1: { default: '', validator, ...extraDefinition },
    propertyName2: { default: '', validator },
    ...extraProps,
  };
}

const makeFx =
  (Schema: any) =>
  (definition: any = undefined, options: any = { timestamps: false }) =>
  () =>
    new Schema(definition, options);

const validator = () => ({ valid: true });
