import { ITimestamp, Private_ISchemaOptions, StringKey } from "../interfaces";

type TimestampKey = StringKey<ITimestamp>;

export class OptionsTool {
  private ts_keys: TimestampKey[];
  private timestamps: ITimestamp;

  constructor(config: Private_ISchemaOptions) {
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
