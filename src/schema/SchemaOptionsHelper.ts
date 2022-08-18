import { ITimestamp, Private_ISchemaOptions } from "./interfaces";

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
