import { makeStringValidator, Schema, validateEmail } from '../../src';

export interface SimpleUser {
  id: string;
  name: string;
  email: string;
  age: number;
}

const okString = (v: any) => ({ valid: true as const, validated: String(v) });
const okNumber = (v: any) => ({ valid: true as const, validated: Number(v) });

export const minimalSchema = () =>
  new Schema<{ value: number }>((b) =>
    b.field(b.required('value').validate(okNumber)),
  ).getModel();

export function manyFieldSchema(count: number, asyncValidators = false) {
  return new Schema<Record<string, number>>((b) => {
    for (let i = 0; i < count; i++) {
      const validator = (v: any) =>
        asyncValidators ? Promise.resolve(okNumber(v)) : okNumber(v);
      b.field(b.required(`field_${i}`).validate(validator));
    }
    return b;
  }).getModel();
}

export function allowListSchema(allowSize: number) {
  const allowed: [number, number, ...number[]] = [
    0,
    1,
    ...Array.from({ length: allowSize - 2 }, (_, i) => i + 2),
  ];
  return new Schema<{ value: number }>((b) =>
    b.field(b.required('value').allow(allowed)),
  ).getModel();
}

export function dependentChainSchema(length: number) {
  return new Schema<Record<string, number>>((b) => {
    b.field(b.lax('field_0', 1));
    for (let i = 1; i < length; i++) {
      b.field(
        b
          .dependent(`field_${i}`, `field_${i - 1}`)
          .default(0)
          .resolve(({ values }) => (values[`field_${i - 1}`] as number) + 1),
      );
    }
    return b;
  }).getModel();
}

export function wideDependencySchema(parentCount: number) {
  const parents = Array.from({ length: parentCount }, (_, i) => `parent_${i}`);
  return new Schema<Record<string, number>>((b) => {
    for (const parent of parents) {
      b.field(b.lax(parent, 1));
    }
    b.field(
      b
        .dependent('child', parents as [string, ...string[]])
        .default(0)
        .resolve(({ values }) =>
          parents.reduce((sum, p) => sum + (values[p] as number), 0),
        ),
    );
    return b;
  }).getModel();
}

export function virtualHeavySchema(count: number) {
  const virtualNames = Array.from(
    { length: count },
    (_, i) => `virtual_${i}`,
  ) as [string, ...string[]];

  return new Schema<{ base: number } & Record<string, number>>((b) => {
    b.field(b.required('base').validate(okNumber));
    for (const name of virtualNames) {
      b.field(
        b
          .virtual(name)
          .validate(okNumber)
          .sanitize(() => 1),
      );
    }
    b.field(
      b
        .dependent('sum', virtualNames)
        .default(0)
        .resolve(({ values }) =>
          virtualNames.reduce((sum, name) => sum + (values[name] as number), 0),
        ),
    );
    return b;
  }).getModel();
}

export function readonlyHeavySchema(count: number) {
  return new Schema<Record<string, string>>((b) => {
    for (let i = 0; i < count; i++) {
      b.field(b.lax(`readonly_${i}`, 'default').readonly());
    }
    return b;
  }).getModel();
}

export function dynamicIgnoreSchema(count: number) {
  return new Schema<Record<string, number>>((b) => {
    for (let i = 0; i < count; i++) {
      b.field(b.lax(`field_${i}`, i).ignore(() => false));
    }
    return b;
  }).getModel();
}

export function optionsReaderSchema(count: number) {
  return new Schema<
    Record<string, number>,
    Record<string, number>,
    { tag: string }
  >((b) => {
    for (let i = 0; i < count; i++) {
      b.field(
        b.required(`field_${i}`).validate((_, ctx) => {
          // Touch options multiple times to stress cloneWithMethods
          const _a = ctx.options.tag;
          const _b = ctx.options.tag;
          const _c = ctx.options.tag;
          return okNumber(_a + _b + _c ? 0 : 0);
        }),
      );
    }
    return b;
  }).getModel();
}

export function userSchema() {
  return new Schema<Partial<SimpleUser>, SimpleUser>((b) =>
    b
      .field(b.lax('id', '').validate(okString))
      .field(b.required('name').validate(okString))
      .field(b.required('email').validate(okString))
      .field(b.required('age').validate(okNumber)),
  ).getModel();
}

// Realistic schema (the TS counterpart of the Rust `main_demo`): a constant
// id resolver, nullable contact fields with validators, a required username
// with an async availability check (reValidate), a virtual password with a
// sanitizer feeding a dependent, dependents of the username, a readonly lax
// field, timestamps, a grouped required rule, post-validation and an
// onSuccess handler.

export interface RealisticUserInput {
  email?: string | null;
  phone?: string | null;
  username: string;
  password: string;
  role?: string;
}

export interface RealisticUser {
  id: number;
  email: string | null;
  phone: string | null;
  username: string;
  slug: string;
  usernameChanges: number;
  passwordHash: string;
  role: string;
  createdAt: Date;
  updatedAt: Date;
}

const takenUsernames = new Set(['taken', 'admin', 'root']);

const validateUsername = makeStringValidator({ min: 3, max: 20 });
const validatePassword = makeStringValidator({ min: 8, max: 64 });

export function realisticUserSchema() {
  let nextId = 0;

  return new Schema<RealisticUserInput, RealisticUser>(
    (b: any) =>
      b
        .field(b.constant('id', () => ++nextId))
        .field(
          b
            .lax('email', null)
            .validate((v: unknown) =>
              v === null ? { valid: true, validated: null } : validateEmail(v),
            ),
        )
        .field(
          b
            .lax('phone', null)
            .validate((v: unknown) =>
              v === null || (typeof v === 'string' && v.length >= 7)
                ? { valid: true, validated: v }
                : { valid: false, reason: 'Invalid phone number' },
            ),
        )
        .field(
          b
            .required('username')
            .validate(validateUsername)
            .reValidate(async (v: string) =>
              takenUsernames.has(v)
                ? { valid: false, reason: `"${v}" is already taken` }
                : { valid: true },
            ),
        )
        .field(
          b
            .virtual('password')
            .validate(validatePassword)
            .sanitize(({ input }: any) => `hashed:${input.password}`),
        )
        .field(
          b
            .dependent('passwordHash', 'password')
            .default('')
            .resolve(({ input }: any) => input.password),
        )
        .field(
          b
            .dependent('slug', 'username')
            .default('')
            .resolve(({ values }: any) => values.username.toLowerCase()),
        )
        .field(
          b
            .dependent('usernameChanges', 'username')
            .default(0)
            .resolve(({ isUpdate, values }: any) =>
              isUpdate ? values.usernameChanges + 1 : 0,
            ),
        )
        .field(b.lax('role', 'member').readonly()),
    {
      timestamps: true,
      required: {
        fields: ['email', 'phone'],
        // only on creation: an update only carries the fields it changes
        handler: ({ input, isUpdate }: any) =>
          !isUpdate && input.email == null && input.phone == null
            ? { email: 'provide an email or a phone number' }
            : undefined,
      },
      postValidate: {
        fields: ['username', 'password'],
        validator: ({ input }: any) =>
          input.password?.includes(input.username)
            ? { password: 'must not contain the username' }
            : undefined,
      },
      onSuccess: () => {},
    } as any,
  ).getModel();
}

export const realisticUserInput: RealisticUserInput = {
  email: 'ada@example.com',
  phone: '123 4567 890',
  username: 'Ada_L',
  password: 'correct horse battery',
};
