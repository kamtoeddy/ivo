import { expect } from 'bun:test';
import { Schema } from '../src/schema';
import { isEqual } from '../src/utils';
import type { ObjectType } from '../src/utils/types';

export {
  expectFailure,
  expectNoFailure,
  findBy,
  getSubObject,
  getValidSchema,
  makeFx,
  validator,
};

function expectFailure(fx: Function, message: string = 'INVALID_SCHEMA') {
  expect(fx).toThrow(message);
}

function expectNoFailure(fx: Function) {
  expect(fx).not.toThrow();
}

function findBy<T>(list: T[], determinant: any): T | undefined {
  return list.find((dt) => {
    const sub = getSubObject(dt as ObjectType, determinant);

    return isEqual(determinant, sub);
  });
}

function getDeepValue(data: ObjectType, key: string): any {
  // @ts-expect-error lol
  return key.split('.').reduce((prev, next) => prev?.[next], data);
}

function getSubObject(obj: ObjectType, sampleSub: ObjectType) {
  const _obj: ObjectType = {},
    keys = Object.keys(sampleSub);

  keys.forEach((key) => {
    _obj[key] = getDeepValue(obj, key);
  });

  return _obj;
}

function getValidSchema(
  extendField1: (b: any) => any = (b) => b,
  extraFields: any[] = [],
) {
  return (b: any) => {
    b.field(extendField1(b.lax('fieldName1', '').validate(validator))).field(
      b.lax('fieldName2', '').validate(validator),
    );

    for (const extraField of extraFields) b.field(extraField);

    return b;
  };
}

const makeFx =
  (builder: (b: any) => any = (b) => b, options: any = { timestamps: false }) =>
  () =>
    new Schema(builder, options);

const validator = () => ({ valid: true });
