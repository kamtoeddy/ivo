import { isEqual } from '../../utils';
import { NS, KeyOf } from '../types';

export { TimeStampTool };

type TimestampKey = KeyOf<NS.Timestamp>;

class TimeStampTool {
  private _keys: TimestampKey[];
  private timestamps: NS.Timestamp;

  constructor(timestamps: NS.Options<any, any, any>['timestamps']) {
    this.timestamps = this._makeTimestamps(timestamps);

    this._keys = Object.keys(this.timestamps).filter(
      (key) => key.length > 0
    ) as TimestampKey[];
  }

  private _makeTimestamps(timestamps: NS.Options<any, any, any>['timestamps']) {
    if (isEqual(timestamps, undefined)) return { createdAt: '', updatedAt: '' };

    let createdAt = 'createdAt',
      updatedAt = 'updatedAt';

    if (!timestamps || timestamps === true)
      return timestamps
        ? { createdAt, updatedAt }
        : { createdAt: '', updatedAt: '' };

    const custom_createdAt = timestamps?.createdAt;
    const custom_updatedAt = timestamps?.updatedAt;

    if (custom_createdAt && typeof custom_createdAt == 'string')
      createdAt = custom_createdAt.trim();

    if (custom_createdAt === false) createdAt = '';

    if (custom_updatedAt && typeof custom_updatedAt == 'string')
      updatedAt = custom_updatedAt.trim();

    if (custom_updatedAt === false) updatedAt = '';

    return { createdAt, updatedAt };
  }

  getKeys() {
    return {
      createdAt: this.timestamps.createdAt,
      updatedAt: this.timestamps.updatedAt
    };
  }

  isTimestampKey = (key: string) => this._keys.includes(key as TimestampKey);

  get withTimestamps() {
    return !!(this.timestamps.createdAt || this.timestamps.updatedAt);
  }
}
