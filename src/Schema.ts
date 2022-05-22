import ApiError from "./utils/ApiError";
import isEqual from "./utils/isEqual";
import validate from "./validate";
import format from "./utils/format";

import { looseObject, stringPropTypes } from "./utils/interfaces";

type looseOjectFunc = (data: any) => looseObject;

type funcArrayObj = () => looseObject;
type funcArrayFunc = () => looseOjectFunc[];
type funcArrayStr = () => string[];

export default class Schema {
  [key: string]: any;

  timestamps: boolean = true;
  createdAt: Date = new Date();
  updatedAt: Date = new Date();

  validations: looseObject = {};

  private getCreateActions: funcArrayFunc = () => [];
  private getCreateProps: funcArrayStr = () => [];
  private getDefaults: funcArrayObj = () => ({});
  private getLinkedUpdates: funcArrayObj = () => ({});
  private getProps: funcArrayStr = () => [];
  private getUpdatables: funcArrayStr = () => [];

  private errors: looseObject = {};
  private updated: looseObject = {};

  constructor({
    createdAt = new Date(),
    timestamps = false,
    updatedAt = new Date(),
  } = {}) {
    this.timestamps = timestamps;
    this.createdAt = createdAt;
    this.updatedAt = updatedAt;
  }

  private _addError = ({ field, errors }: { field: string; errors: any[] }) => {
    const _error = this.errors?.[field];

    if (!_error) return (this.errors[field] = [...errors]);

    this.errors[field] = [...this.errors[field], ...errors];
  };

  private _getCloneObject = (toReset: string[] = []) => {
    const defaults = this.getDefaults();
    const props = this.getProps();

    return props.reduce((values: looseObject, next) => {
      values[next] = toReset.includes(next)
        ? defaults[next] ?? this[next]
        : this[next];
      return values;
    }, {});
  };

  private _getCreateObject = () => {
    const createProps = this.getCreateProps();
    const defaults = this.getDefaults();
    const props = this.getProps();

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

  private _getDependencies = (prop: string): looseOjectFunc[] =>
    this.getLinkedUpdates()[prop] ?? [];

  private _isErroneous = () => Object.keys(this.errors).length > 0;

  private _returnErrors() {
    throw new ApiError({ message: "Validation error", payload: this.errors });
  }

  private _postCreateActions = (data = {}) => {
    const actions = this.getCreateActions();

    actions.forEach((action) => (data = { ...data, ...action(data) }));

    return data;
  };

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
    if (!this.getProps().includes(prop))
      return { valid: false, messages: ["Invalid property"] };

    let valid = true,
      messages: string[] = [],
      validated = value;

    const validateFx = this.validations[prop];

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
    const _linkedKeys = Object.keys(this.getLinkedUpdates() ?? {});
    const _linkedUpdates: string[] = [];
    const _updatables = this.getUpdatables();

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
