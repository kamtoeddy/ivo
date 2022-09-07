import { sort, toArray } from "../utils/functions";
import { ObjectType } from "../utils/interfaces";
import { Schema as ns, StringKey } from "./interfaces";
import { defaultOptions, SchemaCore } from "./SchemaCore";

export class Schema<T extends ObjectType> extends SchemaCore<T> {
  constructor(
    propDefinitions: ns.PropertyDefinitions<T>,
    options: ns.Options = defaultOptions
  ) {
    super(propDefinitions, options);
  }

  get options() {
    return this._options;
  }

  get propDefinitions() {
    return this._propDefinitions;
  }

  private _useExtensionOptions = <T extends ObjectType>(
    options: ns.ExtensionOptions<T>
  ) => {
    const remove = toArray(options?.remove ?? []);

    remove?.forEach((prop) => delete this._propDefinitions?.[prop]);

    return this;
  };

  extend = <U extends ObjectType>(
    parent: Schema<U>,
    options: ns.ExtensionOptions<U> = { remove: [] }
  ) => {
    this._propDefinitions = {
      ...parent.propDefinitions,
      ...this._propDefinitions,
    } as ns.PropertyDefinitions<T>;

    return this._useExtensionOptions(options);
  };

  getModel = () => Model.build(this);
}

class Model<T extends ObjectType> extends SchemaCore<T> {
  constructor(schema: Schema<T>, values: Partial<T>) {
    super(schema.propDefinitions, schema.options);
    this.setValues(values);
  }

  static build<U extends ObjectType>(schema: Schema<U>) {
    return function Builder(values: Partial<U>): Model<U> {
      return new Model(schema, values);
    };
  }

  private setValues(values: Partial<T>) {
    const keys = Object.keys(values).filter(
      (key) =>
        this._helper.isTimestampKey(key) ||
        this._isProp(key) ||
        this._isSideEffect(key)
    ) as StringKey<T>[];

    sort(keys).forEach((key) => (this.values[key] = values[key]));

    this._initContext();
  }

  clone = async (options: ns.CloneOptions<T> = { reset: [] }) => {
    return this._getCloneObject(toArray(options.reset).filter(this._isProp));
  };

  create = async () => this._getCreateObject();

  update = async (changes: Partial<T>) => {
    const updated = {};

    const toUpdate = Object.keys(changes ?? {}).filter((prop) =>
      this._isUpdatable(prop)
    ) as StringKey<T>[];

    const validations = toUpdate.map((prop) => {
      return this._validateAndSet(updated, prop, changes[prop]);
    });

    await Promise.all(validations);

    if (this._isErroneous()) this._throwError();

    const linkedProps = toUpdate.filter((prop) => !this._isSideEffect(prop));
    const sideEffects = toUpdate.filter(this._isSideEffect);

    await this._resolveLinked(linkedProps, updated, "onUpdate");

    await this._resolveLinked(sideEffects, updated, "onUpdate");

    if (!Object.keys(updated).length) this._throwError("Nothing to update");

    return this._useConfigProps(updated, true);
  };
}

// interface Model<T> {
//   clone: (options?: ns.CloneOptions<T>) => Promise<T>;
//   create: () => Promise<T>;
//   update: (changes: Partial<T>) => Promise<Partial<T>>;
// }

// export class Schema<T extends ObjectType> extends SchemaCore<T> {
//   constructor(
//     propDefinitions: ns.PropertyDefinitions<T>,
//     options: ns.Options = defaultOptions
//   ) {
//     super(propDefinitions, options);
//   }

//   get options() {
//     return this._options;
//   }

//   get propDefinitions() {
//     return this._propDefinitions;
//   }

//   private _useExtensionOptions = <T extends ObjectType>(
//     options: ns.ExtensionOptions<T>
//   ) => {
//     const remove = toArray(options?.remove ?? []);

//     remove?.forEach((prop) => delete this._propDefinitions?.[prop]);

//     return this;
//   };

//   extend = <U extends ObjectType>(
//     parent: Schema<U>,
//     options: ns.ExtensionOptions<U> = { remove: [] }
//   ) => {
//     this._propDefinitions = {
//       ...parent.propDefinitions,
//       ...this._propDefinitions,
//     } as ns.PropertyDefinitions<T>;

//     return this._useExtensionOptions(options);
//   };

//   getModel = () => {
//     const model = new ModelTool(this);

//     return (values: Partial<T>) => {
//       model.setValues(values);

//       return {
//         clone: (options?: ns.CloneOptions<T>) => model.clone(options),

//         create: () => model.create(),

//         update: (changes: Partial<T>) => model.update(changes),
//       } as Model<T>;
//     };
//   };
// }

// class ModelTool<T extends ObjectType> extends SchemaCore<T> {
//   constructor(schema: Schema<T>) {
//     super(schema.propDefinitions, schema.options);
//   }

//   setValues(values: Partial<T>) {
//     const keys = Object.keys(values).filter(
//       (key) =>
//         this._helper.isTimestampKey(key) ||
//         this._isProp(key) ||
//         this._isSideEffect(key)
//     ) as StringKey<T>[];

//     this.values = {};

//     sort(keys).forEach((key) => (this.values[key] = values[key]));
//     this._initContext();
//   }

//   clone = async (options: ns.CloneOptions<T> = { reset: [] }) => {
//     return this._getCloneObject(toArray(options.reset).filter(this._isProp));
//   };

//   create = async () => this._getCreateObject();

//   update = async (changes: Partial<T>) => {
//     const updated = {};

//     const toUpdate = Object.keys(changes ?? {}).filter((prop) =>
//       this._isUpdatable(prop)
//     ) as StringKey<T>[];

//     const validations = toUpdate.map((prop) => {
//       return this._validateAndSet(updated, prop, changes[prop]);
//     });

//     await Promise.all(validations);

//     if (this._isErroneous()) this._throwError();

//     const linkedProps = toUpdate.filter((prop) => !this._isSideEffect(prop));
//     const sideEffects = toUpdate.filter(this._isSideEffect);

//     await this._resolveLinked(linkedProps, updated, "onUpdate");

//     await this._resolveLinked(sideEffects, updated, "onUpdate");

//     if (!Object.keys(updated).length) this._throwError("Nothing to update");

//     return this._useConfigProps(updated, true);
//   };
// }
