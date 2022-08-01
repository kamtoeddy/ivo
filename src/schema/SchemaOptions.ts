import { ITimestamp, Private_ISchemaOptions } from "./interfaces";

export class SchemaOptions {
  private keys: ITimestamp;

  constructor(config: Private_ISchemaOptions) {
    const { timestamps } = config;

    this.keys = timestamps;
  }

  getCreateKey = () => this.keys.createdAt;
  getUpdateKey = () => this.keys.updatedAt;

  get withTimestamps() {
    return !!(this.keys.createdAt || this.keys.updatedAt);
  }
}
