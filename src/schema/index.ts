import { asArray } from "../utils/asArray";
import {
  IExtensionOptions,
  ISchemaOptions,
  PropDefinitionRules,
} from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";

export class Schema extends SchemaCore<Schema> {
  constructor(
    propDefinitions: PropDefinitionRules,
    options: ISchemaOptions = defaultOptions
  ) {
    super(propDefinitions, options);
    this._checkPropDefinitions();
  }

  private _useExtensionOptions = (options: IExtensionOptions) => {
    const remove = asArray(options.remove);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);
  };

  extend = (parent: Schema, options: IExtensionOptions = { remove: [] }) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    };

    this._useExtensionOptions(options);

    return this;
  };
}
