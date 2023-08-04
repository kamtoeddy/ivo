import type { ISchema, StringKey } from '../types'

type TimestampKey = StringKey<ISchema.Timestamp>

export class OptionsTool {
  private _keys: TimestampKey[]
  private timestamps: ISchema.Timestamp

  constructor(config: ISchema.PrivateOptions) {
    const { timestamps } = config

    this.timestamps = timestamps
    this._keys = Object.keys(timestamps).filter(
      (key) => key.length > 0
    ) as TimestampKey[]
  }

  getKeys = () => {
    return {
      createdAt: this.timestamps.createdAt,
      updatedAt: this.timestamps.updatedAt
    }
  }

  isTimestampKey = (key: string) => this._keys.includes(key as TimestampKey)

  get withTimestamps() {
    return !!(this.timestamps.createdAt || this.timestamps.updatedAt)
  }
}
