import type { ObjectType } from '../../utils';
import { BUILD } from '../types';
import type { DefaultFieldErrorMetadata } from '../utils';
import { type BlankConstantBuilder, ConstantBuilder } from './constants';
import { type BlankDependentBuilder, DependentBuilder } from './dependents';
import { type BlankLaxBuilder, LaxBuilder } from './lax';
import { type BlankRequiredBuilder, RequiredBuilder } from './required';
import { type BlankVirtualBuilder, VirtualBuilder } from './virtual';

export { createFieldBuilder, materializeFieldBuilders };

/**
 * Prototype of a Rust-style typestate builder for the "dependent" field
 * nature (`rs/src/schema/fields/dependents.rs`'s `DependentFieldBuilder`).
 *
 * `default` and `dependsOn` can be supplied in either order, but `resolve`
 * only becomes callable once both are set — each stage is a distinct
 * interface exposing only the methods valid at that stage, so calling things
 * out of order is a compile error rather than a runtime one.
 * `readonly`/`onDelete`/`onSuccess` are only available on the finished
 * builder, matching the Rust impl.
 *
 * There is no user-facing `.build()`: the finished builder exposes its
 * resolved config only through the `BUILD` symbol hook, which isn't part of
 * `Buildable`'s public surface. So the chain's result can be dropped
 * directly into a `Definitions` object literal, and `Schema` resolves it
 * internally via `materializeFieldBuilders` - the only place that imports
 * `BUILD` as a value.
 */

/**
 * Binds `Input`/`Output`/`CtxOptions` once (mirroring how they're pinned on
 * `Schema<Input, Output, CtxOptions>`), then infers `K` per-field from the
 * property name passed to `.dependent(...)`.
 */
function createFieldBuilder<
  Input,
  Output = Input,
  CtxOptions extends ObjectType = {},
  Metadata = DefaultFieldErrorMetadata,
>() {
  return {
    constant<K extends keyof Output>(
      _fieldName: K,
    ): BlankConstantBuilder<Output[K], Input, Output, CtxOptions> {
      return new ConstantBuilder<Output[K], Input, Output, CtxOptions>();
    },
    dependent<K extends keyof Output>(
      _fieldName: K,
    ): BlankDependentBuilder<K, Input, Output, CtxOptions> {
      return new DependentBuilder<K, Input, Output, CtxOptions>();
    },
    lax<K extends keyof Output & keyof Input>(
      _fieldName: K,
    ): BlankLaxBuilder<
      (Input & Output)[K],
      Input,
      Output,
      CtxOptions,
      Metadata
    > {
      return new LaxBuilder<
        (Input & Output)[K],
        Input,
        Output,
        CtxOptions,
        Metadata
      >();
    },
    required<K extends keyof Output & keyof Input>(
      _fieldName: K,
    ): BlankRequiredBuilder<
      (Input & Output)[K],
      Input,
      Output,
      CtxOptions,
      Metadata
    > {
      return new RequiredBuilder<
        (Input & Output)[K],
        Input,
        Output,
        CtxOptions,
        Metadata
      >();
    },
    virtual<K extends string>(
      _fieldName: K,
    ): BlankVirtualBuilder<any, Input, Output, CtxOptions, Metadata> {
      return new VirtualBuilder<any, Input, Output, CtxOptions, Metadata>();
    },
  };
}

function isFieldBuilder(value: unknown): value is { [BUILD]: () => unknown } {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as { [BUILD]?: unknown })[BUILD] === 'function'
  );
}

/**
 * Called by `Schema` before processing a `Definitions` object, so users
 * never get (or need) a way to resolve a field-builder chain themselves -
 * mirrors how the Rust schema macro resolves `impl BuildableFieldConfig`
 * under the hood. This is the only place in the codebase that imports
 * `BUILD` as a value, which is what keeps it out of reach elsewhere.
 */
function materializeFieldBuilders<T extends ObjectType>(definitions: T): T {
  const result: ObjectType = { ...definitions };

  for (const key of Object.keys(result)) {
    const value = result[key];

    if (isFieldBuilder(value)) result[key] = value[BUILD]();
  }

  return result as T;
}
