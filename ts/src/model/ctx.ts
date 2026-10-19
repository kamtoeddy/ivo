/**
 * The contexts handed to handlers. Each is a snapshot of the operation's
 * state when it was built, kept in private fields and read through getters
 * shared on the prototype -- unlike getters on an object literal, building
 * one allocates no functions. Callers freeze them after construction (a
 * subclass's fields are only added once the base constructor has returned).
 */

import { cloneWithMethods } from '../utils';

export {
  ConstantCtx,
  CreateCtx,
  InitResolverCtx,
  ReadonlyCreateCtx,
  ReadonlyUpdateCtx,
  UpdateCtx,
  UpdateResolverCtx,
};

type AnyObject = Record<PropertyKey, any>;
type UpdateOptions = (options: any) => unknown;

class InitResolverCtx {
  #input: AnyObject;
  #rawInput: AnyObject;
  #options: AnyObject;
  #updateOptions: UpdateOptions;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    options: AnyObject,
    updateOptions: UpdateOptions,
  ) {
    this.#input = input;
    this.#rawInput = rawInput;
    this.#options = options;
    this.#updateOptions = updateOptions;
  }

  get input() {
    return this.#input;
  }

  get rawInput() {
    return this.#rawInput;
  }

  get options() {
    return cloneWithMethods(this.#options);
  }

  get updateOptions() {
    return this.#updateOptions;
  }
}

class ConstantCtx extends InitResolverCtx {
  #values: AnyObject;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    options: AnyObject,
    updateOptions: UpdateOptions,
    values: AnyObject,
  ) {
    super(input, rawInput, options, updateOptions);
    this.#values = values;
  }

  get values() {
    return this.#values;
  }
}

class UpdateResolverCtx extends InitResolverCtx {
  #previousValues: AnyObject;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    options: AnyObject,
    updateOptions: UpdateOptions,
    previousValues: AnyObject,
  ) {
    super(input, rawInput, options, updateOptions);
    this.#previousValues = previousValues;
  }

  get previousValues() {
    return this.#previousValues;
  }
}

class ReadonlyCreateCtx {
  #input: AnyObject;
  #rawInput: AnyObject;
  #values: AnyObject;
  #options: AnyObject;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    values: AnyObject,
    options: AnyObject,
  ) {
    this.#input = input;
    this.#rawInput = rawInput;
    this.#values = values;
    this.#options = options;
  }

  get changes() {
    return null;
  }

  get input() {
    return this.#input;
  }

  get rawInput() {
    return this.#rawInput;
  }

  get isUpdate() {
    return false as const;
  }

  get previousValues() {
    return null;
  }

  get values() {
    return this.#values;
  }

  get options() {
    return cloneWithMethods(this.#options);
  }
}

class ReadonlyUpdateCtx {
  #input: AnyObject;
  #rawInput: AnyObject;
  #changes: AnyObject;
  #previousValues: AnyObject;
  #options: AnyObject;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    changes: AnyObject,
    previousValues: AnyObject,
    options: AnyObject,
  ) {
    this.#input = input;
    this.#rawInput = rawInput;
    this.#changes = changes;
    this.#previousValues = previousValues;
    this.#options = options;
  }

  get changes() {
    return this.#changes;
  }

  get input() {
    return this.#input;
  }

  get rawInput() {
    return this.#rawInput;
  }

  get isUpdate() {
    return true as const;
  }

  get previousValues() {
    return this.#previousValues;
  }

  get values() {
    return Object.assign({}, this.#previousValues, this.#changes);
  }

  get options() {
    return cloneWithMethods(this.#options);
  }
}

class CreateCtx extends ReadonlyCreateCtx {
  #updateOptions: UpdateOptions;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    values: AnyObject,
    options: AnyObject,
    updateOptions: UpdateOptions,
  ) {
    super(input, rawInput, values, options);
    this.#updateOptions = updateOptions;
  }

  get updateOptions() {
    return this.#updateOptions;
  }
}

class UpdateCtx extends ReadonlyUpdateCtx {
  #updateOptions: UpdateOptions;

  constructor(
    input: AnyObject,
    rawInput: AnyObject,
    changes: AnyObject,
    previousValues: AnyObject,
    options: AnyObject,
    updateOptions: UpdateOptions,
  ) {
    super(input, rawInput, changes, previousValues, options);
    this.#updateOptions = updateOptions;
  }

  get updateOptions() {
    return this.#updateOptions;
  }
}
