export { Merge, RealType };

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T;

type RealType_<T> = T extends (...args: any) => infer I ? I : T;

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>;
} & {};

export type OmitIndexSignature<ObjectType> = {
  [K in keyof ObjectType as {} extends Record<K, unknown>
    ? never
    : K]: ObjectType[K];
};

type SimpleMerge<I, O> = {
  [K in keyof I as K extends keyof O ? never : K]: I[K];
} & O;

type Merge<I, O> = I extends O
  ? O extends I
    ? I
    : EnforceOptional<
        SimpleMerge<PickIndexSignature<I>, PickIndexSignature<O>> &
          SimpleMerge<OmitIndexSignature<I>, OmitIndexSignature<O>>
      >
  : EnforceOptional<
      SimpleMerge<PickIndexSignature<I>, PickIndexSignature<O>> &
        SimpleMerge<OmitIndexSignature<I>, OmitIndexSignature<O>>
    >;

type PickIndexSignature<ObjectType> = {
  [K in keyof ObjectType as {} extends Record<K, unknown>
    ? K
    : never]: ObjectType[K];
};

// Returns `never` if the key is optional otherwise return the key type.
type RequiredFilter<I, K extends keyof I> = undefined extends I[K]
  ? I[K] extends undefined
    ? K
    : never
  : K;

// Returns `never` if the key is required otherwise return the key type.
type OptionalFilter<T, K extends keyof T> = undefined extends T[K]
  ? T[K] extends undefined
    ? never
    : K
  : never;

export type EnforceOptional<ObjectType> = RealType<
  {
    [K in keyof ObjectType as RequiredFilter<ObjectType, K>]: ObjectType[K];
  } & {
    [K in keyof ObjectType as OptionalFilter<ObjectType, K>]?: Exclude<
      ObjectType[K],
      undefined
    >;
  }
>;
