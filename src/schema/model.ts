import { Schema } from ".";
import { asArray } from "../utils/asArray";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import { SchemaCloneOptions } from "./interfaces";
import { SchemaCore } from "./SchemaCore";

export class Model<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema, values: Partial<T>) {
    super(schema.propDefinitions, schema.options);

    // this order of assignment is key
    this.props = this._getProps();
    this.defaults = this._getDefaults();

    this.setValues(values);
  }

  private setValues(values: Partial<T>) {
    Object.keys(values).forEach((key) => {
      if (this._isProp(key) || this._isSideEffect(key))
        this.values[key as keyof T] = values[key];
    });
  }

  static build<U extends ObjectType>(schema: Schema) {
    return function Builder(values: Partial<U>) {
      return new Model(schema, values);
    };
  }

  clone = async (options: SchemaCloneOptions = { reset: [] }) => {
    return this._getCloneObject(asArray(options.reset).filter(this._isProp));
  };

  create = async () => {
    const obj = await this._getCreateObject();

    if (this._isErroneous()) this._throwErrors();

    return obj;
  };

  update = async (changes: Partial<T>) => {
    this.updated = {};

    const toUpdate = Object.keys(changes ?? {});

    // iterate through validated values and get only changed fields
    // amongst the schema's updatable properties
    const updatables = toUpdate.filter((prop) => this._isUpdatable(prop));
    const propsWithUpdateListeners = toUpdate.filter(
      (prop) =>
        (!updatables.includes(prop) &&
          this._getAllListeners(prop, "onUpdate").length) ||
        this._isSideEffect(prop)
    );

    const validations = updatables.map(async (prop) => {
      const validationResults = await this.validate(prop, changes[prop]);

      return { prop, validationResults };
    });

    const results = await Promise.all(validations);

    for (const { prop, validationResults } of results) {
      const { reasons, valid, validated } = validationResults;

      if (!valid) {
        this.error.add(prop, reasons);
        continue;
      }

      const hasChanged = !isEqual(this.values[prop], validated);

      if (!hasChanged) continue;

      this.updated[prop as keyof T] = validated;

      await this._resolveLinkedValue(this.updated, prop, validated, "onUpdate");
    }

    await this._resolveLinked(
      propsWithUpdateListeners,
      this.updated,
      changes,
      "onUpdate"
    );

    if (this._isErroneous()) this._throwErrors();

    const updatedKeys = this._sort(Object.keys(this.updated));
    if (!updatedKeys.length) this._throwErrors("Nothing to update");

    const updated: T = { ...this.updated } as T;

    this._resetContext();
    this.updated = {};

    updatedKeys.forEach(
      (key: string) => (this.updated[key as keyof T] = updated[key])
    );

    return this._useConfigProps(this.updated, true);
  };
}

export const makeModel = Model.build;
