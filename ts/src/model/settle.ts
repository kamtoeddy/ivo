/**
 * Helpers that run a phase's handlers like the `Promise.all(Settled)` +
 * `Promise.try` pipelines they replace -- every handler is called right away,
 * in order, and a synchronous throw counts as a rejection -- but only go
 * async when some handler actually returned a promise. A phase made only of
 * sync handlers then completes synchronously, without allocating a promise
 * per handler or waiting on microtasks.
 */

export type { MaybePromise, Settled };
export { allOrThrow, isThenable, mapMaybe, settleAll, settleEach };

type MaybePromise<T> = T | Promise<T>;

type Settled<T> = { ok: true; value: T } | { ok: false; error: unknown };

function isThenable<T>(value: unknown): value is PromiseLike<T> {
  return (
    value !== null &&
    (typeof value === 'object' || typeof value === 'function') &&
    typeof (value as PromiseLike<T>).then === 'function'
  );
}

/** applies `fn` to `value`, or to what it resolves to */
function mapMaybe<T, R>(
  value: T | PromiseLike<T>,
  fn: (value: T) => R,
): MaybePromise<R> {
  return isThenable<T>(value) ? Promise.resolve(value).then(fn) : fn(value);
}

// Sync throws caught by `call` are wrapped in a marker object registered
// here, so a handler that happens to return `{ ok: false, ... }` itself is
// never mistaken for one.
const settledErrors = new WeakSet<object>();

function isSettledError(
  value: unknown,
): value is { ok: false; error: unknown } {
  return (
    typeof value === 'object' && value !== null && settledErrors.has(value)
  );
}

function call<T>(
  fn: () => T | PromiseLike<T>,
): T | PromiseLike<T> | { ok: false; error: unknown } {
  try {
    return fn();
  } catch (error) {
    const marker = { ok: false as const, error };
    settledErrors.add(marker);
    return marker;
  }
}

/**
 * Calls `run(item)` for every item, then hands each outcome to `onSettled`:
 * sync outcomes right away (in order), async ones as they settle -- like
 * `Promise.allSettled(items.map(async (item) => onSettled(await run(item))))`.
 * Errors thrown by `onSettled` are swallowed, as `allSettled` would.
 */
function settleEach<T, R>(
  items: Iterable<T>,
  run: (item: T) => R | PromiseLike<R>,
  onSettled: (item: T, outcome: Settled<R>) => void,
): MaybePromise<void> {
  const sync: [T, Settled<R>][] = [];
  const pending: [T, PromiseLike<R>][] = [];

  for (const item of items) {
    let result: R | PromiseLike<R>;

    try {
      result = run(item);
    } catch (error) {
      sync.push([item, { ok: false, error }]);
      continue;
    }

    if (isThenable<R>(result)) pending.push([item, result]);
    else sync.push([item, { ok: true, value: result }]);
  }

  const settle = (item: T, outcome: Settled<R>) => {
    try {
      onSettled(item, outcome);
    } catch {}
  };

  for (const [item, outcome] of sync) settle(item, outcome);

  if (!pending.length) return;

  return Promise.all(
    pending.map(([item, promise]) =>
      Promise.resolve(promise).then(
        (value) => settle(item, { ok: true, value }),
        (error) => settle(item, { ok: false, error }),
      ),
    ),
  ).then(() => {});
}

/**
 * Calls every function and returns all outcomes in order, once all have
 * settled -- like `Promise.allSettled(fns.map((fn) => Promise.try(fn)))`.
 */
function settleAll<T>(
  fns: readonly (() => T | PromiseLike<T>)[],
): MaybePromise<Settled<T>[]> {
  const results = fns.map(call);

  if (!results.some(isThenable))
    return results.map((r) =>
      isSettledError(r) ? r : { ok: true, value: r as T },
    );

  return Promise.all(
    results.map((r) =>
      isThenable<T>(r)
        ? Promise.resolve(r).then(
            (value): Settled<T> => ({ ok: true, value }),
            (error): Settled<T> => ({ ok: false, error }),
          )
        : isSettledError(r)
          ? r
          : { ok: true as const, value: r as T },
    ),
  );
}

/**
 * Calls every function and returns their results in order, or throws the
 * first error -- like `Promise.all(fns.map(async (fn) => fn()))`: every
 * function is still called even when an earlier one throws.
 */
function allOrThrow<T>(
  fns: readonly (() => T | PromiseLike<T>)[],
): MaybePromise<T[]> {
  const results = fns.map(call);

  if (!results.some(isThenable)) {
    for (const r of results) if (isSettledError(r)) throw r.error;

    return results as T[];
  }

  return Promise.all(
    results.map((r) =>
      isSettledError(r) ? Promise.reject(r.error) : (r as T | PromiseLike<T>),
    ),
  );
}
