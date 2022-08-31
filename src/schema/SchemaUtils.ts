import { toArray } from "../utils/functions";
import {
  ITimestamp,
  Private_ISchemaOptions,
  ResponseInput,
  ValidatorResponse,
} from "./interfaces";

export class SchemaOptionsHelper {
  private keys: ITimestamp;

  constructor(config: Private_ISchemaOptions) {
    const { timestamps } = config;

    this.keys = timestamps;
  }

  getCreateKey = () => this.keys.createdAt;
  getUpdateKey = () => this.keys.updatedAt;
  isTimestampKey = (key: string) =>
    [this.keys.createdAt, this.keys.updatedAt].includes(key);

  get withTimestamps() {
    return !!(this.keys.createdAt || this.keys.updatedAt);
  }
}

export const makeResponse = <T = undefined>({
  reason,
  reasons,
  valid,
  validated,
}: ResponseInput): ValidatorResponse<T> => {
  if (valid) return { reasons: [], valid, validated };

  if (reasons) reasons = [...toArray(reasons)];
  else reasons = [];

  if (reason) reasons = [...reasons, ...toArray(reason)];

  return { reasons, valid, validated: undefined };
};
