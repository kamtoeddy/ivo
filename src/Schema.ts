import ApiError from "./utils/ApiError";
import isEqual from "./utils/isEqual";

import { looseObject } from "./utils/interfaces";
import { belongsTo } from "./utils/functions";

type looseObjectFunc = (...args: any) => looseObject;

type funcArrayObj = () => looseObject;
type funcArrayFunc = () => looseObjectFunc[];
type funcArrayStr = () => string[];
// type propType = () => Array<any> | Boolean | Date | Number | Object | String;

type validationResponse = {
  messages?: string[];
  valid: boolean;
  validated?: any;
};
type propValidatorFunc = (...args: any) => validationResponse;

interface propDefinitionType {
  [key: string]: {
    default?: any;
    onCreate?: looseObjectFunc[];
    onUpdate?: looseObjectFunc[];
    readonly?: boolean;
    required?: boolean;
    sideEffect: boolean;
    shouldInit?: boolean;
    // type: propType;
    validator?: propValidatorFunc;
  };
}

interface initOptionsType {
  timestamp?: boolean;
}

export default class Schema {
  [key: string]: any;

  private propDefinitions: propDefinitionType = {};
  private options: initOptionsType = { timestamp: false };

  private errors: looseObject = {};
  private updated: looseObject = {};

  constructor(propsDefinitions: propDefinitionType, options: initOptionsType) {
    this.propDefinitions = propsDefinitions;
    this.options = options;

    if (!this._hasEnoughProps())
      throw new ApiError({ message: "Invalid properties", statusCode: 500 });
  }

  private _addError = ({ field, errors }: { field: string; errors: any[] }) => {
    const _error = this.errors?.[field];

    if (!_error) return (this.errors[field] = [...errors]);

    this.errors[field] = [...this.errors[field], ...errors];
  };

  private _canInit = (prop: string): boolean => {
    const propDef = this.propDefinitions[prop];

    if (!propDef) return false;

    const { readonly, required } = propDef;

    if (!readonly && !required) return false;
    // if (!readonly && !required && !this._hasDefault(prop)) return false;

    return belongsTo(propDef?.shouldInit, [true, undefined]);
  };

  private _getCloneObject = (toReset: string[] = []) => {
    const defaults = this._getDefaults();
    const props = this._getProps();

    return props.reduce((values: looseObject, next) => {
      values[next] = toReset.includes(next)
        ? defaults[next] ?? this[next]
        : this[next];
      return values;
    }, {});
  };

  private _getContext = (): looseObject => {
    let context: looseObject = {};

    this._getProps().forEach((prop) => (context[prop] = this[prop]));

    return { ...context, ...this.updated };
  };

  private _getCreateActions: funcArrayFunc = () => {
    let _actions: looseObjectFunc[] = [];
    const props = this._getProps();

    for (let prop of props) {
      let propActions = this.propDefinitions[prop]?.onCreate ?? [];

      propActions = propActions.filter(
        (action) => typeof action === "function"
      );

      if (propActions?.length) _actions = [..._actions, ...propActions];
    }

    return _actions;
  };

  private _getCreateObject = () => {
    const createProps = this._getCreateProps();
    const defaults = this._getDefaults();
    const props = this._getProps();

    return props.reduce((values: looseObject, prop) => {
      const checkLax = this._isLaxProp(prop) && this.hasOwnProperty(prop);

      if (createProps.includes(prop) || checkLax) {
        const {
          valid,
          validated,
          messages: errors,
        } = this.validate({ prop, value: this[prop] });

        if (valid) return { ...values, [prop]: validated };

        this._addError({ field: prop, errors });
      } else {
        values[prop] = defaults[prop];
      }

      return values;
    }, {});
  };

  /**
   * create props are required or readonly props
   * @returns
   */
  private _getCreateProps: funcArrayStr = () => {
    const createProps = [];
    const props = this._getProps();

    for (let prop of props) if (this._canInit(prop)) createProps.push(prop);

    return this._sort(createProps);
  };

  private _getDefaults: funcArrayObj = () => {
    const defaults: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      const _default = this.propDefinitions[prop]?.default;

      if (_default !== undefined) defaults[prop] = _default;
    }

    return defaults;
  };

  private _getLinkedMethods = (prop: string): looseObjectFunc[] => {
    const methods = this._isSideEffect(prop)
      ? this.propDefinitions[prop]?.onUpdate
      : this._getLinkedUpdates()[prop];

    return methods ?? [];
  };

  private _getLinkedUpdates: funcArrayObj = () => {
    const _linkedUpdates: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      let _updates = this.propDefinitions[prop]?.onUpdate ?? [];

      _updates = _updates.filter((action) => this._isFunction(action));

      if (_updates?.length) _linkedUpdates[prop] = _updates;
    }

    return _linkedUpdates;
  };

  private _getProps: funcArrayStr = () => {
    let props: string[] = Object.keys(this.propDefinitions);

    props = props.filter((prop) => {
      const propDef = this.propDefinitions[prop];

      if (typeof propDef !== "object") return false;

      return (
        this._hasSomeOf(propDef, ["default", "readonly", "required"]) ||
        this._isLaxProp(prop)
      );
    });

    return this._sort(props);
  };

  private _getUpdatables: funcArrayStr = () => {
    const updatebles = [];
    const props = this._getProps();

    for (let prop of props) {
      const readonly = this.propDefinitions[prop]?.readonly;
      if (!readonly || (readonly && !this._hasChanged(prop)))
        updatebles.push(prop);
    }

    return this._sort(updatebles);
  };

  private _hasChanged = (prop: string) => {
    const propDef = this.propDefinitions[prop];

    if (!propDef) return false;

    return !isEqual(propDef.default, this?.[prop]);
  };

  private _hasDefault = (prop: string) => {
    const propDef = this.propDefinitions[prop];

    if (!propDef) return false;

    return !isEqual(propDef.default, undefined);
  };

  private _hasEnoughProps = (): boolean => this._getProps().length > 0;

  private _hasSomeOf = (obj: looseObject, props: string[]): boolean => {
    for (let prop of props) if (Object(obj).hasOwnProperty(prop)) return true;

    return false;
  };

  private _isErroneous = () => Object.keys(this.errors).length > 0;

  private _isFunction = (obj: any): boolean => typeof obj === "function";

  private _isLaxProp = (prop: string): boolean => {
    const propDef = this.propDefinitions[prop];

    if (!propDef) return false;

    if (!this._hasDefault(prop)) return false;

    const { readonly, required, shouldInit } = propDef;

    if (readonly || required) return false;

    return belongsTo(shouldInit, [true, undefined]);
  };

  private _isProp = (prop: string): boolean => this._getProps().includes(prop);

  private _isSideEffect = (prop: string): boolean => {
    const propDef = this.propDefinitions[prop];

    if (!propDef) return false;

    const { sideEffect, onUpdate, validator } = propDef;

    if (!this._isFunction(validator)) return false;

    const methods = onUpdate?.filter((method) => this._isFunction(method));

    if (!methods?.length) return false;

    return sideEffect === true;
  };

  private _postCreateActions = (data: looseObject = {}): looseObject => {
    const actions = this._getCreateActions();

    actions.forEach((action) => (data = { ...data, ...action(data) }));

    return data;
  };

  private _throwErrors(message: string = "Validation Error"): void {
    const payload = this.errors;
    this.errors = {};

    throw new ApiError({ message, payload });
  }

  private _sort = (data: any[]): any[] => data.sort((a, b) => (a < b ? -1 : 1));

  clone = ({ toReset = [] } = { toReset: [] }) => {
    return this._postCreateActions(this._getCloneObject(toReset));
  };

  create = () => {
    let obj = this._getCreateObject();

    if (this.options?.timestamp) {
      obj = { ...obj, createdAt: new Date(), updatedAt: new Date() };
    }

    if (this._isErroneous()) this._throwErrors();

    return this._postCreateActions(obj);
  };

  getValidations = (): looseObject => {
    const validations: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      const validator = this.propDefinitions[prop]?.validator;

      if (typeof validator === "function") validations[prop] = validator;
    }

    return validations;
  };

  validate = ({ prop = "", value }: { prop: string; value: any }) => {
    const isSideEffect = this._isSideEffect(prop);

    if (!this._isProp(prop) && !isSideEffect)
      return { valid: false, messages: ["Invalid property"] };

    let valid = true,
      messages: string[] = [],
      validated = value;

    const validateFx = isSideEffect
      ? this.propDefinitions[prop].validator
      : this.getValidations()[prop];

    if (!validateFx && typeof value === "undefined") {
      return { valid: false, messages: ["Invalid value"] };
    }

    if (validateFx) {
      const {
        reason,
        valid: _v,
        validated: _va,
      } = validateFx(value, this._getContext());

      valid = _v;

      messages = [reason];

      if (_va) validated = _va;
    }

    return { messages, valid, validated };
  };

  /**
   * Function to validate schemas during an update operation
   * @param {object} changes Object holding the opdated values
   * @returns An object containing validated changes
   */
  update = (changes: looseObject = {}) => {
    this.updated = {};

    const toUpdate = Object.keys(changes);
    const _linkedKeys = Object.keys(this._getLinkedUpdates());
    const _updatables = this._getUpdatables();

    // iterate through validated values and get only changed fields
    // amongst the schema's updatable properties
    toUpdate.forEach((prop: string) => {
      const isLinked = _linkedKeys.includes(prop),
        isSideEffect = this._isSideEffect(prop),
        isUpdatable = _updatables.includes(prop);

      if (!isSideEffect && !isLinked && !isUpdatable) return;

      const {
        valid,
        validated,
        messages: errors,
      } = this.validate({
        prop,
        value: changes[prop],
      });

      const hasChanged = !isEqual(this[prop], validated);

      if ((valid && !isSideEffect && hasChanged) || (valid && isSideEffect)) {
        if (isUpdatable) this.updated[prop] = validated;

        if (isLinked || isSideEffect) {
          const methods = this._getLinkedMethods(prop);
          const context = this._getContext();

          if (isSideEffect) context[prop] = validated;

          methods.forEach(
            (cb) => (this.updated = { ...this.updated, ...cb(context) })
          );
        }

        return;
      }

      if (!valid) this._addError({ field: prop, errors });
    });

    if (this._isErroneous()) this._throwErrors();

    // get the number of properties updated
    // and deny update if none was modified
    const canUpdate = Object.keys(this.updated).length;
    if (!canUpdate) this._throwErrors("Nothing to update");

    if (this.options?.timestamp) this.updated.updatedAt = new Date();

    return this.updated;
  };
}

function Model(this: Schema, values: looseObject) {
  Object.keys(values).forEach((key) => (this[key] = values[key]));
  return this;
}

export const makeModel = (schema: Schema) => {
  return function (args: looseObject) {
    return Model.call(schema, args);
  };
};
