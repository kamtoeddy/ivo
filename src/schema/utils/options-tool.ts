import type { ISchema, StringKey } from "../interfaces";

type TimestampKey = StringKey<ISchema.Timestamp>;

export class OptionsTool {
  private ts_keys: TimestampKey[];
  private timestamps: ISchema.Timestamp;

  constructor(config: ISchema.PrivateOptions) {
    const { timestamps } = config;

    this.timestamps = timestamps;
    this.ts_keys = Object.keys(timestamps).filter(
      (key) => key.length > 0
    ) as TimestampKey[];
  }

  getCreateKey = () => this.timestamps.createdAt;
  getUpdateKey = () => this.timestamps.updatedAt;
  isTimestampKey = (key: string) => this.ts_keys.includes(key as TimestampKey);

  get withTimestamps() {
    return !!(this.timestamps.createdAt || this.timestamps.updatedAt);
  }
}
