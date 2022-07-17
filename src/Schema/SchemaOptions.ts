import { ITimestamp, Private_ISchemaOptions } from "./interfaces";

export class SchemaOptions {
  private keys: ITimestamp;

  constructor(config: Private_ISchemaOptions) {
    const { timestamp } = config;

    this.keys = timestamp;
  }

  getCreateKey = () => this.keys.createdAt;
  getUpdateKey = () => this.keys.updatedAt;

  get withTimestamp() {
    return !!(this.keys.createdAt || this.keys.updatedAt);
  }
}
