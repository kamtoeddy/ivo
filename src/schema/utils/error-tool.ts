import { isEqual, isPropertyOf, FieldKey, sortKeys } from '../../utils';
import {
  ErrorPayload,
  FieldError,
  ValidationErrorMessage,
  IErrorTool,
} from './types';

export { DefaultErrorTool };

class DefaultErrorTool<PayloadKeys extends FieldKey = FieldKey>
  implements IErrorTool<{ payload: ErrorPayload<PayloadKeys> }>
{
  private _payload: ErrorPayload<PayloadKeys> = {};

  constructor(private message: ValidationErrorMessage) {}

  get data() {
    return { message: this.message, payload: sortKeys(this._payload) };
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

    const { reasons, metadata } = value;

    reasons.forEach((reason) => {
      if (!currentValues.reasons.includes(reason))
        currentValues.reasons.push(reason);
    });

    if (metadata && !isEqual(currentValues?.metadata, metadata))
      currentValues.metadata = {
        ...(currentValues?.metadata ?? {}),
        ...metadata,
      };

    this._payload[field] = currentValues;

    return this;
  }

  setMessage(message: ValidationErrorMessage) {
    this.message = message;

    return this;
  }
}
