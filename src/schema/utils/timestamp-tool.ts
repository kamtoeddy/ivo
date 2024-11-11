import { isEqual } from "../../utils";
import { NS, KeyOf } from "../types";

export { TimeStampTool };

type TimestampKey = KeyOf<NS.Timestamp>;

const IS_UPDATED_AT_TIMESTAMP_NULLABLE_BY_DEFAULT = true;

class TimeStampTool {
  private _keys: TimestampKey[];
  private timestamps: NS.Timestamp;
  private nullable: boolean;

  constructor(timestamps: NS.Options<unknown, unknown, unknown>["timestamps"]) {
    this.timestamps = this._makeTimestamps(timestamps);
    this.nullable =
      typeof timestamps === "object" &&
      typeof timestamps?.updatedAt === "object"
        ? (timestamps.updatedAt.nullable ??
          IS_UPDATED_AT_TIMESTAMP_NULLABLE_BY_DEFAULT)
        : IS_UPDATED_AT_TIMESTAMP_NULLABLE_BY_DEFAULT;

    this._keys = Object.keys(this.timestamps).filter(
      (key) => key.length > 0,
    ) as TimestampKey[];
  }

  private _makeTimestamps(
    timestamps: NS.Options<unknown, unknown, unknown>["timestamps"],
  ) {
    if (isEqual(timestamps, undefined)) return { createdAt: "", updatedAt: "" };

    let createdAt = "createdAt",
      updatedAt = "updatedAt";

    if (!timestamps || timestamps === true)
      return timestamps
        ? { createdAt, updatedAt }
        : { createdAt: "", updatedAt: "" };

    const custom_createdAt = timestamps?.createdAt;
    const custom_updatedAt =
      typeof timestamps?.updatedAt === "object"
        ? timestamps?.updatedAt?.key
        : timestamps?.updatedAt;

    if (custom_createdAt && typeof custom_createdAt == "string")
      createdAt = custom_createdAt.trim();

    if (custom_createdAt === false) createdAt = "";

    if (custom_updatedAt && typeof custom_updatedAt == "string")
      updatedAt = custom_updatedAt.trim();

    if (custom_updatedAt === false) updatedAt = "";

    return { createdAt, updatedAt };
  }

  getKeys() {
    return {
      createdAt: this.timestamps.createdAt,
      updatedAt: this.timestamps.updatedAt,
    };
  }

  isTimestampKey = (key: string) => this._keys.includes(key as TimestampKey);

  get isNullable() {
    return this.nullable;
  }

  get withTimestamps() {
    return !!(this.timestamps.createdAt || this.timestamps.updatedAt);
  }
}
