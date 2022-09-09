import { sort, sortKeys, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { isEqual } from "../utils/isEqual";
import { LifeCycles, Schema as ns, StringKey } from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";
import { makeResponse } from "./utils";

export class Schema<T extends ObjectType> extends SchemaCore<T> {
  constructor(
    propDefinitions: ns.PropertyDefinitions<T>,
    options: ns.Options = defaultOptions
  ) {
    super(propDefinitions, options);
  }

  get options() {
    return this._options;
  }

  get propDefinitions() {
    return this._propDefinitions;
  }

  private _useExtensionOptions = <T extends ObjectType>(
    options: ns.ExtensionOptions<T>
  ) => {
    const remove = toArray(options?.remove ?? []);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);

    return this;
  };

  extend = <U extends ObjectType>(
    parent: Schema<U>,
    options: ns.ExtensionOptions<U> = { remove: [] }
  ) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    } as ns.PropertyDefinitions<T>;

    return this._useExtensionOptions(options);
  };

  getModel = () => new Model(new ModelTool(this));
}

class ModelTool<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema<T>) {
    super(schema.propDefinitions, schema.options);
  }

  private _getCreatePropsWithListeners = () => {
    const listeners = [];

    for (let prop of Array.from(this.props))
      if (this._getAllListeners(prop, "onCreate")?.length) listeners.push(prop);

    return listeners;
  };

  private _isUpdatable = (prop: string) => {
    if (this._isConstant(prop)) return false;

    if (this._isSideEffect(prop)) return true;

    if (!this._isProp(prop) || this._isDependentProp(prop)) return false;

    const { readonly } = this._getDefinition(prop);

    const _default = this._getDefaultValue(prop, false);

    return !readonly || (readonly && isEqual(_default, this.values[prop]));
  };

  private _isUpdatableInCTX = (
    prop: string,
    value: any,
    context: ObjectType = this._getContext()
  ) => {
    return this._isConstant(prop) || !this._isProp(prop)
      ? false
      : !isEqual(value, context?.[prop]);
  };

  private _resolveLinked = async (
    operationData: Partial<T>,
    props: StringKey<T>[],
    lifeCycle: LifeCycles.Rule
  ) => {
    const listenersUpdates = props.map((prop) => {
      return this._resolveLinkedProps(operationData, prop, lifeCycle);
    });

    await Promise.all(listenersUpdates);
  };

  private _resolveLinkedProps = async (
    operationData: Partial<T> = {},
    prop: StringKey<T>,
    lifeCycle: LifeCycles.Rule
  ) => {
    const listeners = this._getAllListeners(prop, lifeCycle);

    if (
      !listeners.length ||
      (lifeCycle === "onUpdate" &&
        !this._isSideEffect(prop) &&
        !this._isUpdatableInCTX(prop, operationData[prop], this.values))
    )
      return;

    for (const listener of listeners) {
      const context = { ...this._getContext(), ...operationData };

      const extra = await listener(context);

      if (typeof extra !== "object") continue;

      const _props = Object.keys(extra) as StringKey<T>[];

      for (let _prop of _props) {
        const _value = extra[_prop];
        const isSideEffect = this._isSideEffect(_prop);

        if (!isSideEffect && !this._isUpdatableInCTX(_prop, _value, context))
          continue;

        await this._validateAndSet(operationData, _prop, _value);

        await this._resolveLinkedProps(operationData, _prop, lifeCycle);
      }
    }
  };

  private _validateAndSet = async (
    operationData: Partial<T> = {},
    prop: StringKey<T>,
    value: any
  ) => {
    let { reasons, valid, validated } = await this.validate(prop, value);

    if (!valid) return this.error.add(prop, reasons);

    if (isEqual(validated, undefined)) validated = value;

    if (!this._isSideEffect(prop)) operationData[prop] = validated;

    this._updateContext({ [prop]: validated } as T);
  };

  private _useConfigProps = (obj: T | Partial<T>, asUpdate = false) => {
    if (!this.optionsTool.withTimestamps) return sortKeys(obj);

    const createdAt = this.optionsTool.getCreateKey(),
      updatedAt = this.optionsTool.getUpdateKey();

    const results = asUpdate
      ? { ...obj, [updatedAt]: new Date() }
      : { ...obj, [createdAt]: new Date(), [updatedAt]: new Date() };

    return sortKeys(results);
  };

  private _handleRequiredBy = () => {
    for (const prop of this.propsRequiredBy) {
      const isRequired = this._getValueBy(prop, "required");
      if (isRequired)
        return this.error.add(prop, this._getValueBy(prop, "requiredError"));
    }
  };

  clone = async (options: ns.CloneOptions<T> = { reset: [] }) => {
    const reset = toArray(options.reset).filter(this._isProp);

    const data = {} as T;

    const sideEffects = Object.keys(this.values).filter(
      this._isSideInit
    ) as StringKey<T>[];

    const props = [...Array.from(this.props), ...sideEffects];

    const validations = props.map((prop) => {
      if (this._isConstant(prop))
        return (data[prop] = this._getValueBy(prop, "value"));

      const isSideEffect = sideEffects.includes(prop);

      if (isSideEffect && !this._isSideInit(prop)) return;

      if (!isSideEffect && reset.includes(prop)) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      const isLaxInit =
        this._isLaxProp(prop) &&
        !isEqual(this.values[prop], this.defaults[prop]);

      const isRequiredInit =
        this._isRequiredBy(prop) && this.values.hasOwnProperty(prop);

      if (
        !isSideEffect &&
        !this._canInit(prop) &&
        !isLaxInit &&
        !isRequiredInit
      ) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] as any } as T);
      }

      return this._validateAndSet(data, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy();

    if (this._isErroneous()) this._throwError();

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(data, linkedProps, "onCreate");

    await this._resolveLinked(data, sideEffects, "onCreate");

    return this._useConfigProps(data) as T;
  };

  create = async () => {
    const data = {} as T;

    const sideEffects = Object.keys(this.values).filter(
      this._isSideInit
    ) as StringKey<T>[];

    const props = [...Array.from(this.props), ...sideEffects];

    const validations = props.map((prop) => {
      if (this._isConstant(prop)) {
        data[prop] = this._getValueBy(prop, "value");

        return this._updateContext({ [prop]: data[prop] } as T);
      }

      const isSideEffect = sideEffects.includes(prop);
      if (isSideEffect && !this._isSideInit(prop)) return;

      const isProvided = this.values.hasOwnProperty(prop);

      const isLaxInit = this._isLaxProp(prop) && isProvided;

      const isRequiredInit = this._isRequiredBy(prop) && isProvided;

      if (
        !isSideEffect &&
        !this._canInit(prop) &&
        !isLaxInit &&
        !isRequiredInit
      ) {
        data[prop] = this._getDefaultValue(prop);

        return this._updateContext({ [prop]: data[prop] } as T);
      }

      return this._validateAndSet(data, prop, this.values[prop]);
    });

    await Promise.all(validations);

    this._handleRequiredBy();

    if (this._isErroneous()) this._throwError();

    const linkedProps = this._getCreatePropsWithListeners();

    await this._resolveLinked(data, linkedProps, "onCreate");

    await this._resolveLinked(data, sideEffects, "onCreate");

    return this._useConfigProps(data) as T;
  };

  setValues(values: Partial<T>) {
    const keys = Object.keys(values).filter(
      (key) =>
        this.optionsTool.isTimestampKey(key) ||
        this._isProp(key) ||
        this._isSideEffect(key)
    ) as StringKey<T>[];

    this.values = {};

    sort(keys).forEach((key) => (this.values[key] = values[key]));

    this._initContext();
  }

  update = async (changes: Partial<T>) => {
    const updated = {} as Partial<T>;

    const toUpdate = Object.keys(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop)
    ) as StringKey<T>[];

    const linkedProps: StringKey<T>[] = [];
    const sideEffects: StringKey<T>[] = [];

    const validations = toUpdate.map(async (prop) => {
      const value = changes[prop];
      let { reasons, valid, validated } = await this.validate(prop, value);

      if (!valid) return this.error.add(prop, reasons);

      if (isEqual(validated, undefined)) validated = value;

      if (isEqual(validated, this.values[prop])) return;

      if (this._isSideEffect(prop)) sideEffects.push(prop);
      else {
        updated[prop] = validated;
        linkedProps.push(prop);
      }

      this._updateContext({ [prop]: validated } as T);
    });

    await Promise.all(validations);

    this._handleRequiredBy();

    if (this._isErroneous()) this._throwError();

    if (!Object.keys(updated).length && !sideEffects.length)
      this._throwError("Nothing to update");

    await this._resolveLinked(updated, linkedProps, "onUpdate");

    await this._resolveLinked(updated, sideEffects, "onUpdate");

    if (!Object.keys(updated).length) this._throwError("Nothing to update");

    return this._useConfigProps(updated, true);
  };

  validate = async <K extends StringKey<T>>(prop: K, value: any) => {
    if (
      this._isConstant(prop) ||
      (!this._isProp(prop) && !this._isSideEffect(prop))
    )
      return { valid: false, reasons: ["Invalid property"] };

    const validator = this._getValidator(prop);

    if (validator)
      return makeResponse<T[K]>(await validator(value, this._getContext()));

    return makeResponse<T[K]>({ valid: true, validated: value });
  };
}

class Model<T extends ObjectType> {
  constructor(private modelTool: ModelTool<T>) {}

  clone = async (
    values: Partial<T>,
    options: ns.CloneOptions<T> = { reset: [] }
  ) => {
    this.modelTool.setValues(values);

    return this.modelTool.clone(options);
  };

  create = async (values: Partial<T>) => {
    this.modelTool.setValues(values);

    return this.modelTool.create();
  };

  update = async (values: Partial<T>, changes: Partial<T>) => {
    this.modelTool.setValues(values);

    return this.modelTool.update(changes);
  };

  validate = this.modelTool.validate;
}
