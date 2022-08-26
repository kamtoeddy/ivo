import { toArray } from "../utils/toArray";
import {
  SchemaExtensionOptions,
  SchemaOptions,
  PropDefinitionRules,
} from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";

export class Schema extends SchemaCore<Schema> {
  constructor(
    propDefinitions: PropDefinitionRules,
    options: SchemaOptions = defaultOptions
  ) {
    super(propDefinitions, options);
    this._checkPropDefinitions();
  }

  private _useExtensionOptions = (options: SchemaExtensionOptions) => {
    const remove = toArray(options.remove);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);
  };

  extend = (
    parent: Schema,
    options: SchemaExtensionOptions = { remove: [] }
  ) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    };

    this._useExtensionOptions(options);

    return this;
  };
}
