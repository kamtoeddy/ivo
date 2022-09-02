import { toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import {
  SchemaExtensionOptions,
  SchemaOptions,
  PropDefinitionRules,
  SchemaCloneOptions,
  StringKeys,
} from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";

export class Schema<T extends ObjectType> extends SchemaCore<T> {
  constructor(
    propDefinitions: PropDefinitionRules<T>,
    options: SchemaOptions = defaultOptions
  ) {
    super(propDefinitions, options);
    this._checkPropDefinitions();
  }

  private _useExtensionOptions = <T extends ObjectType>(
    options: SchemaExtensionOptions<T>
  ) => {
    const remove = toArray(options?.remove ?? []);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);

    return this;
  };

  extend = <U extends ObjectType>(
    parent: Schema<U>,
    options: SchemaExtensionOptions<U> = { remove: [] }
  ) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    } as PropDefinitionRules<T>;

    return this._useExtensionOptions(options);
  };

  getModel = () => Model.build<T>(this);
}

class Model<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema<T>, values: Partial<T>) {
    super(schema.propDefinitions, schema.options);

    // this order of assignment is precious
    this.props = this._getProps();
    this.defaults = this._getDefaults();

    this.setValues(values);
  }

  static build<U extends ObjectType>(schema: Schema<U>) {
    return function Builder(values: Partial<U>): Model<U> {
      return new Model(schema, values);
    };
  }

  private _getDefaults = () => {
    const defaults: Partial<T> = {};

    for (let prop of this.props) {
      const _default = this._propDefinitions[prop]?.default;

      if (!isEqual(_default, undefined)) defaults[prop] = _default;
    }

    return defaults;
  };

  private setValues(values: Partial<T>) {
    const keys = Object.keys(values).filter(
      (key) =>
        this._helper.isTimestampKey(key) ||
        this._isProp(key) ||
        this._isSideEffect(key)
    ) as StringKeys<T>[];

    this._sort(keys).forEach((key) => (this.values[key] = values[key]));
  }

  clone = async (options: SchemaCloneOptions<T> = { reset: [] }) => {
    return this._getCloneObject(toArray(options.reset).filter(this._isProp));
  };

  create = async () => {
    const obj = await this._getCreateObject();

    if (this._isErroneous()) this._throwErrors();

    return obj;
  };

  update = async (changes: Partial<T>) => {
    this.updated = {};

    const toUpdate = Object.keys(changes ?? {}) as StringKeys<T>[];

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
