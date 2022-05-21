const ApiError = require("./ApiError");
import isEqual from "./isEqual";

const isValid = require("./isValid");
const { format } = require("./format");

class Schema {
  constructor(
    { createdAt = new Date(), timestamps = true, updatedAt = new Date() } = {
      createdAt: new Date(),
      timestamps: true,
      updatedAt: new Date(),
    }
  ) {
    this.timestamps = timestamps;
    this.createdAt = createdAt;
    this.updatedAt = updatedAt;

    this.getCreateActions = () => [];
    this.getCreateProps = () => [];
    this.getDefaults = () => ({});
    this.getLinkedUpdates = () => ({});
    this.getProps = () => [];
    this.getUpdatables = () => [];

    this.errors = {};
    this.updated = {};
  }

  _addError = ({ field, errors }) => {
    const _error = this.errors[field];

    if (!_error) return (this.errors[field] = [...errors]);

    this.errors[field] = [...this.errors, ...errors];
  };

  _getCloneObject = (toReset = []) => {
    const defaults = this.getDefaults();
    const props = this.getProps();

    return props.reduce((values, next) => {
      values[next] = toReset.includes(next)
        ? defaults[next] ?? this[next]
        : this[next];
      return values;
    }, {});
  };

  _getCreateObject = () => {
    const createProps = this.getCreateProps();
    const defaults = this.getDefaults();
    const props = this.getProps();

    return props.reduce((values, prop) => {
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

  _getDependencies = (prop) => this.getLinkedUpdates()[prop] ?? [];

  _isErroneous = () => Object.keys(this.errors).length;

  _returnErrors() {
    throw new ApiError({ message: "Validation error", payload: this.errors });
  }

  _postCreateActions = (data = {}) => {
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

  validations = {};

  validate = ({ prop = "", value }) => {
    if (!this.getProps().includes(prop))
      return { valid: false, messages: ["Invalid property"] };

    let valid = true,
      messages = [],
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

  validate_Boolean = (value) => {
    const { valid, reason } = isValid.boolean(value);
    return { messages: [reason], valid, validated: value };
  };

  validate_CompanyCode = (value) => this.validate_String(value);

  validate_String = (value, options = {}) => {
    const { valid, reason } = isValid.string(value, options);

    if (!valid) return { valid, messages: [reason] };

    const validated = format(value, "string", { trim: true, getSub: true });

    return { valid: true, validated };
  };

  validate_UnitCode = (val) => this.validate_String(val);

  validate_UnitName = (val) => this.validate_String(val);

  /**
   * Function to validate schemas during an update operation
   * @param {object} changes Object holding the opdated values
   * @returns An object containing validated changes
   */
  update = (changes = {}) => {
    const changesProps = Object.keys(changes);
    const _linkedKeys = Object.keys(this.getLinkedUpdates() ?? {});
    const _linkedUpdates = [];
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
    toUpdate.forEach((prop) => {
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
    _linkedUpdates.forEach((prop) => {
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

module.exports = { Schema };
