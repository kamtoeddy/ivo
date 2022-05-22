import ApiError from "./utils/ApiError";
import isEqual from "./utils/isEqual";
import validate from "./validate";
import format from "./utils/format";

import { looseObject, stringPropTypes } from "./utils/interfaces";

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

interface basePropsType {
  [key: string]: {
    default?: any;
    onCreate?: looseObjectFunc[];
    onUpdate?: looseObjectFunc[];
    readonly?: boolean;
    required?: boolean;
    // type: propType;
    validator?: propValidatorFunc;
  };
}

interface initOptionsType {
  timestamps?: boolean;
}

export default class Schema {
  [key: string]: any;

  protected baseProps: basePropsType = {};
  protected _options: initOptionsType;

  timestamps: boolean = false;
  createdAt: Date = new Date();
  updatedAt: Date = new Date();

  validations: looseObject = {};

  protected errors: looseObject = {};
  protected updated: looseObject = {};

  constructor(
    props: basePropsType,
    options: initOptionsType = { timestamps: false }
  ) {
    this.baseProps = props;
    this._options = options;

    if (!this._hasEnoughProps())
      throw new ApiError({ message: "Invalid properties", statusCode: 500 });
  }

  _getCreateActions: funcArrayFunc = () => {
    let _actions: looseObjectFunc[] = [];
    const props = this._getProps();

    for (let prop of props) {
      const propActions = this.baseProps[prop]?.onCreate;

      if (propActions?.length) _actions = [..._actions, ...propActions];
    }

    return _actions;
  };

  /**
   * create props are required or readonly props
   * @returns
   */
  _getCreateProps: funcArrayStr = () => {
    const createProps = [];
    const props = this._getProps();

    for (let prop of props) {
      const value = this.baseProps[prop];
      if (value?.required || value?.readonly) createProps.push(prop);
    }

    return this._sort(createProps);
  };

  _getDefaults: funcArrayObj = () => {
    const defaults: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      const _default = this.baseProps[prop]?.default;

      if (_default !== undefined) defaults[prop] = _default;
    }

    return defaults;
  };

  _getLinkedUpdates: funcArrayObj = () => {
    const _linkedUpdates: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      const _updates = this.baseProps[prop]?.onUpdate;

      if (_updates?.length) _linkedUpdates[prop] = _updates;
    }

    return _linkedUpdates;
  };

  _getProps: funcArrayStr = () => {
    const keys = Object.keys(this.baseProps);

    const props: string[] = keys.filter((key) => {
      const value = this.baseProps[key];

      if (typeof value !== "object") return false;

      return this._hasSomeOf(value, [
        "default",
        "onCreate",
        "onUpdate",
        "readonly",
        "required",
        "validator",
      ]);
    });

    return this._sort(props);
  };

  _getUpdatables: funcArrayStr = () => {
    const updatebles = [];
    const props = this._getProps();

    for (let prop of props) {
      if (!this.baseProps[prop]?.readonly) updatebles.push(prop);
    }

    return this._sort(updatebles);
  };

  protected _addError = ({
    field,
    errors,
  }: {
    field: string;
    errors: any[];
  }) => {
    const _error = this.errors?.[field];

    if (!_error) return (this.errors[field] = [...errors]);

    this.errors[field] = [...this.errors[field], ...errors];
  };

  protected _getCloneObject = (toReset: string[] = []) => {
    const defaults = this._getDefaults();
    const props = this._getProps();

    return props.reduce((values: looseObject, next) => {
      values[next] = toReset.includes(next)
        ? defaults[next] ?? this[next]
        : this[next];
      return values;
    }, {});
  };

  protected _getCreateObject = () => {
    const createProps = this._getCreateProps();
    const defaults = this._getDefaults();
    const props = this._getProps();

    return props.reduce((values: looseObject, prop) => {
      if (createProps.includes(prop)) {
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

  protected _getDependencies = (prop: string): looseObjectFunc[] =>
    this._getLinkedUpdates()[prop] ?? [];

  _getValidations = (): looseObject => {
    const validations: looseObject = {};
    const props = this._getProps();

    for (let prop of props) {
      const validator = this.baseProps[prop]?.validator;

      if (validator !== undefined) validations[prop] = validator;
    }

    return validations;
  };

  protected _isErroneous = () => Object.keys(this.errors).length > 0;

  protected _returnErrors() {
    throw new ApiError({ message: "Validation error", payload: this.errors });
  }

  protected _postCreateActions = (data = {}) => {
    const actions = this._getCreateActions();

    actions.forEach((action) => (data = { ...data, ...action(data) }));

    return data;
  };

  protected _hasEnoughProps = () => this._getProps().length > 0;

  protected _hasSomeOf = (obj: looseObject, props: string[]): boolean => {
    for (let prop of props) if (Object(obj).hasOwnProperty(prop)) return true;

    return false;
  };

  protected _sort = (data: any[]): any[] =>
    data.sort((a, b) => (a < b ? -1 : 1));

  clone = ({ toReset = [] } = { toReset: [] }) => {
    return this._postCreateActions(this._getCloneObject(toReset));
  };

  create = () => {
    let obj = this._getCreateObject();

    if (this.timestamps) {
      obj = {
        ...obj,
        timestamps: true,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
    }

    if (this._isErroneous()) this._returnErrors();

    return this._postCreateActions(obj);
  };

  validate = ({ prop = "", value }: { prop: string; value: any }) => {
    if (!this._getProps().includes(prop))
      return { valid: false, messages: ["Invalid property"] };

    let valid = true,
      messages: string[] = [],
      validated = value;

    const validateFx = this._getValidations()[prop];

    if (!validateFx && typeof value === "undefined") {
      return { valid: false, messages: ["Invalid value"] };
    }

    if (validateFx) {
      const result = validateFx(value);

      valid = result.valid;
      const _messages = result?.messages ?? [];
      messages = [..._messages];

      if (result.hasOwnProperty("validated")) validated = result.validated;
    }

    return { messages, valid, validated };
  };

  validate_Boolean = (value: any) => {
    validate;
    const { valid, reason } = validate.isBooleanOk(value);
    return { messages: [reason], valid, validated: value };
  };

  validate_String = (value: any, options: stringPropTypes) => {
    const { valid, reason } = validate.isStringOk(value, options);

    if (!valid) return { valid, messages: [reason] };

    const validated = format(value, "string", { trim: true, getSub: true });

    return { valid: true, validated };
  };

  /**
   * Function to validate schemas during an update operation
   * @param {object} changes Object holding the opdated values
   * @returns An object containing validated changes
   */
  update = (changes: looseObject = {}) => {
    const changesProps = Object.keys(changes);
    const _linkedKeys = Object.keys(this._getLinkedUpdates() ?? {});
    const _linkedUpdates: string[] = [];
    const _updatables = this._getUpdatables();

    const toUpdate = changesProps.filter((prop) => {
      if (_linkedKeys.includes(prop)) _linkedUpdates.push(prop);
      return _updatables.includes(prop);
    });

    // reject operation if no valid property to update
    // or no linked property to update
    if (!toUpdate.length && !_linkedUpdates.length) {
      throw new ApiError({ message: "Nothing to update" });
    }

    this.updated = {};

    // iterate through validated values and get only changed fields
    // amongst the schema's updatable properties
    toUpdate.forEach((prop: string) => {
      const {
        valid,
        validated,
        messages: errors,
      } = this.validate({
        prop,
        value: changes[prop],
      });

      if (valid && !isEqual(this?.[prop], validated))
        return (this.updated[prop] = validated);

      if (!valid) this._addError({ field: prop, errors });
    });

    // iterate through validated values and
    // linked updates to attach like full name
    // which depends on first name and last name
    // full name gets updated only when either gets updated
    _linkedUpdates.forEach((prop: string) => {
      const { valid, validated } = this.validate({
        prop,
        value: changes[prop],
      });

      if (!valid) return;

      const dependencies = this._getDependencies(prop);
      dependencies.forEach((cb) => cb(validated));
    });

    if (this._isErroneous()) this._returnErrors();

    // get the number of properties updated
    // and deny update if none was modified
    const canUpdate = Object.keys(this.updated).length;
    if (!canUpdate) throw new ApiError({ message: "Nothing to update" });

    if (this?.timestamps) {
      const timeNow = new Date();
      this.updated.updatedAt = timeNow;
      if (!this.createdAt) this.updated.createdAt = timeNow;
    }

    return this.updated;
  };
}

function Model(this: any, { ...values }) {
  // console.log(this, values);
  console.log(this, 1);
  Object.keys(values).forEach((key) => (this[key] = values[key]));

  return this;
}

export const buildModel = (schema: any) => {
  Model.call(schema, { ...Object.keys(schema) });
  return Model;
};
