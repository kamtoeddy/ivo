import { sort, toArray } from "../utils/functions";
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
      const { default: _default } = this._getDefinition(prop);

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

    sort(keys).forEach((key) => (this.values[key] = values[key]));

    this._initContext();
  }

  clone = async (options: SchemaCloneOptions<T> = { reset: [] }) => {
    return this._getCloneObject(toArray(options.reset).filter(this._isProp));
  };

  create = async () => this._getCreateObject();

  update = async (changes: Partial<T>) => {
    this.updated = {};

    const toUpdate = Object.keys(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop)
    ) as StringKeys<T>[];

    const validations = toUpdate.map((prop) => {
      return this._validateAndSet(this.updated, prop, changes[prop]);
    });

    await Promise.all(validations);

    if (this._isErroneous()) this._throwErrors();

    const linkedProps = toUpdate.filter((prop) => !this._isSideEffect(prop));
    const sideEffects = toUpdate.filter(this._isSideEffect);

    await this._resolveLinked(linkedProps, this.updated, "onUpdate");

    await this._resolveLinked(sideEffects, this.updated, "onUpdate");

    if (!Object.keys(this.updated).length)
      this._throwErrors("Nothing to update");

    this._resetContext();

    return this._useConfigProps(this.updated, true);
  };
}
