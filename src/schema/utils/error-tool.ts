import { isEqual, isPropertyOf, FieldKey, sortKeys } from '../../utils';
import {
  ErrorPayload,
  FieldError,
  ValidationErrorMessage,
  IErrorTool
} from './types';

export { DefaultErrorTool, ValidationError };

class DefaultErrorTool<PayloadKeys extends FieldKey = FieldKey>
  implements IErrorTool<{ payload: ErrorPayload<PayloadKeys> }>
{
  private _payload: ErrorPayload<PayloadKeys> = {};

  constructor(public message: ValidationErrorMessage) {}

  get data() {
    return {
      message: this.message!,
      payload: sortKeys(this._payload)
    };
  }

  get error() {
    return new ValidationError(this.data);
  }

  get fields() {
    return Object.keys(this._payload);
  }

  get isLoaded() {
    return Object.keys(this._payload).length > 0;
  }

  private _has = (field: PayloadKeys) => isPropertyOf(field, this._payload);

  add(field: PayloadKeys, value: FieldError) {
    if (!this._has(field)) {
      this._payload[field] = value;

      return this;
    }

    const currentValues = this._payload[field]!;

    const { reasons = [], metadata } = value;

    reasons.forEach((reason) => {
      if (!currentValues.reasons.includes(reason))
        currentValues.reasons.push(reason);
    });

    if (metadata && !isEqual(currentValues.metadata, metadata))
      currentValues.metadata = {
        ...(currentValues?.metadata ?? {}),
        ...metadata
      };

    this._payload[field] = currentValues;

    return this;
  }

  setMessage(message: ValidationErrorMessage) {
    this.message = message;

    return this;
  }
}

class ValidationError<OutputKeys extends FieldKey> extends Error {
  payload: ErrorPayload<OutputKeys> = {};

  constructor({
    message,
    payload = {}
  }: {
    message: ValidationErrorMessage;
    payload?: ErrorPayload<OutputKeys>;
  }) {
    super(message);
    this.payload = payload;
  }
}
