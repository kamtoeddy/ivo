import { Schema } from ".";
import { toArray } from "../utils/toArray";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import { SchemaCloneOptions } from "./interfaces";
import { SchemaCore } from "./SchemaCore";

export class Model<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema, values: Partial<T>) {
    super(schema.propDefinitions, schema.options);

    // this order of assignment is precious
    this.props = this._getProps();
    this.defaults = this._getDefaults();

    this.setValues(values);
  }

  static build<U extends ObjectType>(schema: Schema) {
    return function Builder(values: Partial<U>) {
      return new Model(schema, values);
    };
  }

  private _getDefaults = () => {
    const defaults: Partial<T> = {};

    for (let prop of this.props) {
      const _default = this._propDefinitions[prop]?.default;

      if (!isEqual(_default, undefined)) defaults[prop as keyof T] = _default;
    }

    return defaults;
  };

  private setValues(values: Partial<T>) {
    const keys = Object.keys(values).filter(
      (key) =>
        this._helper.isTimestampKey(key) ||
        this._isProp(key) ||
        this._isSideEffect(key)
    );

    this._sort(keys).forEach(
      (key) => (this.values[key as keyof T] = values[key])
    );
  }

  clone = async (options: SchemaCloneOptions = { reset: [] }) => {
    return this._getCloneObject(toArray(options.reset).filter(this._isProp));
  };

  create = async () => {
    const obj = await this._getCreateObject();

    if (this._isErroneous()) this._throwErrors();

    return obj;
  };

  update = async (changes: Partial<T>) => {
    this.updated = {};

    const toUpdate = Object.keys(changes ?? {});

    const updatables = toUpdate.filter((prop) => this._isUpdatable(prop));
    const linkedProps = toUpdate.filter(
      (prop) =>
        (!updatables.includes(prop) &&
          this._getAllListeners(prop, "onUpdate").length) ||
        this._isSideEffect(prop)
    );

    await this._resolveLinked(updatables, this.updated, changes, "onUpdate");

    await this._resolveLinked(linkedProps, this.updated, changes, "onUpdate");

    if (this._isErroneous()) this._throwErrors();

    if (!Object.keys(this.updated).length)
      this._throwErrors("Nothing to update");

    this._resetContext();

    return this._useConfigProps(this.updated, true);
  };
}

export const makeModel = Model.build;
