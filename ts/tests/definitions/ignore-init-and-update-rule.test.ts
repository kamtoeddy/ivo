import {
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  it,
  mock,
} from "bun:test";

import { type ReadonlyIvoContext, Schema } from "../../src";
import { expectFailure, expectNoFailure, makeFx, validator } from "../_utils";

describe("ignore", () => {
  describe("valid", () => {
    it("should accept ignore + default", () => {
      const fxn = makeFx((b, m) =>
        b.field(
          m
            .lax("fieldName")
            .default(true)
            .ignore(() => false),
        ),
      );

      expectNoFailure(fxn);

      fxn();
    });

    it("should accept ignore + virtual", () => {
      const fxn = makeFx((b, m) =>
        b
          .field(
            m
              .dependent("dependent", "fieldName")
              .default(true)
              .resolve(validator as never),
          )
          .field(
            m
              .virtual("fieldName")
              .validate(validator)
              .ignore(() => false),
          ),
      );

      expectNoFailure(fxn);

      fxn();
    });

    describe("behaviour", () => {
      it("should ignore accordingly", async () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .lax("isBlocked")
                .default(false)
                .ignore(({ input: { env } }: any) => env === "dev"),
            )
            .field(m.lax("env").default("dev"))
            .field(m.lax("laxField").default(0)),
        ).getModel();

        const { data } = await Model.create({ env: "dev", isBlocked: true });

        expect(data).toMatchObject({
          env: "dev",
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create({
            env: "Lol",
            isBlocked: true,
          });

          expect(data).toMatchObject({
            env: "Lol",
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            {
              env: "Lol",
              isBlocked: true,
              laxField: 0,
            },
            { env: "dev", isBlocked: "updated" },
          );
          expect(data).toEqual({ env: "dev" });
        }

        {
          const { data } = await Model.update(
            {
              env: "dev",
              isBlocked: true,
              laxField: 0,
            },
            { env: "Lol", isBlocked: "updated" },
          );

          expect(data).toEqual({ env: "Lol", isBlocked: "updated" });
        }
      });

      it("should not trigger validators of ignored properties", async () => {
        const validator = () => true;

        const mockedValidator = mock(validator);

        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .lax("isBlocked")
                .default(false)
                .validate(mockedValidator as never)
                .ignore(({ input: { env } }: any) => env === "dev"),
            )
            .field(m.lax("env").default("dev"))
            .field(m.lax("laxField").default(0)),
        ).getModel();

        const { data } = await Model.create({ env: "dev", isBlocked: true });

        expect(mockedValidator).toBeCalledTimes(0);

        expect(data).toMatchObject({
          env: "dev",
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create({
            env: "Lol",
            isBlocked: true,
          });

          expect(mockedValidator).toBeCalledTimes(1);

          expect(data).toMatchObject({
            env: "Lol",
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            {
              env: "Lol",
              isBlocked: true,
              laxField: 0,
            },
            { env: "dev", isBlocked: "updated" },
          );
          expect(mockedValidator).toBeCalledTimes(1);
          expect(data).toEqual({ env: "dev" });
        }

        {
          const { data } = await Model.update(
            {
              env: "dev",
              isBlocked: true,
              laxField: 0,
            },
            { env: "Lol", isBlocked: "updated" },
          );

          expect(mockedValidator).toBeCalledTimes(2);
          expect(data).toEqual({ env: "Lol", isBlocked: "updated" });
        }
      });

      it("should properly handle ignored properties even when not provided", async () => {
        const validator = () => true;

        const mockedValidator = mock(validator);

        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .lax("isBlocked")
                .default(false)
                .validate(mockedValidator as never)
                .ignore(({ input: { env } }: any) => env === "dev"),
            )
            .field(m.lax("env").default("dev"))
            .field(m.lax("laxField").default(0)),
        ).getModel();

        const { data } = await Model.create({ env: "dev" });

        expect(mockedValidator).toBeCalledTimes(0);

        expect(data).toMatchObject({
          env: "dev",
          isBlocked: false,
          laxField: 0,
        });

        {
          const { data } = await Model.create({
            env: "Lol",
            isBlocked: true,
          });

          expect(mockedValidator).toBeCalledTimes(1);

          expect(data).toMatchObject({
            env: "Lol",
            isBlocked: true,
            laxField: 0,
          });
        }

        {
          const { data } = await Model.update(
            {
              env: "Lol",
              isBlocked: true,
              laxField: 0,
            },
            { env: "dev", isBlocked: "updated" },
          );
          expect(mockedValidator).toBeCalledTimes(1);
          expect(data).toEqual({ env: "dev" });
        }

        {
          const { data } = await Model.update(
            {
              env: "dev",
              isBlocked: true,
              laxField: 0,
            },
            { env: "Lol", isBlocked: "updated" },
          );

          expect(mockedValidator).toBeCalledTimes(2);
          expect(data).toEqual({ env: "Lol", isBlocked: "updated" });
        }
      });
    });
  });

  // "should reject ignore & no default" discarded: `.ignore()` isn't
  // available on `LaxBuilder` until `.default()` has been called, so a
  // field with `ignore` but no default is structurally unrepresentable.
  describe("invalid", () => {
    it("should reject ingnore !(() => boolean)", () => {
      const values = [
        undefined,
        1,
        {},
        null,
        [],
        true,
        false,
        "yes",
        "false",
        "true",
      ];

      for (const ignore of values) {
        const fxn = makeFx((b, m) =>
          b.field(
            m
              .lax("fieldName")
              .default(true)
              .ignore(ignore as never),
          ),
        );

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                '"ignore" must be a function that returns a boolean',
              ]),
            }),
          );
        }
      }
    });

    it("should reject ignore + (ignoreInit | ignoreUpdate)", () => {
      // Entries setting `ignoreInit`/`ignoreUpdate` to a literal `false`
      // are excluded: `.ignoreInit()`/`.ignoreUpdate()` only ever set
      // `true` or a resolver (`resolver ?? true`), so there's no builder
      // call that produces a literal `false` value for either rule.
      const values = [
        { ignoreInit: true },
        { ignoreUpdate: true },
        { ignoreInit: true, ignoreUpdate: () => true },
        { ignoreInit: () => true, ignoreUpdate: true },
        { ignoreInit: () => true, ignoreUpdate: () => true },
      ];

      for (const config of values) {
        const fxn = makeFx((b, m) => {
          let builder: any = m
            .lax("fieldName")
            .default(true)
            .ignore(() => true);

          if ("ignoreInit" in config)
            builder =
              config.ignoreInit === true
                ? builder.ignoreInit()
                : builder.ignoreInit(config.ignoreInit as never);

          if ("ignoreUpdate" in config)
            builder =
              config.ignoreUpdate === true
                ? builder.ignoreUpdate()
                : builder.ignoreUpdate(config.ignoreUpdate as never);

          return b.field(builder);
        });

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                '"ignore" cannot be used with "ignoreInit" or "ignoreUpdate"',
              ]),
            }),
          );
        }
      }
    });
  });
});

describe("ignoreInit", () => {
  describe("valid", () => {
    it("should accept ignoreInit(false) + default", () => {
      const fxn = makeFx((b, m) =>
        b.field(m.lax("fieldName").default(true).ignoreInit()),
      );

      expectNoFailure(fxn);

      fxn();
    });

    it("should accept ignoreInit: () => boolean + default", () => {
      const values = [() => true, () => false];

      for (const ignoreInit of values) {
        const fxn = makeFx((b, m) =>
          b.field(m.lax("fieldName").default(true).ignoreInit(ignoreInit)),
        );

        expectNoFailure(fxn);

        fxn();
      }
    });

    describe("behaviour", () => {
      const Model = new Schema<any>((b, m) =>
        b
          .field(
            m
              .lax("isBlocked")
              .default(false)
              .ignoreInit((input: any) => input?.env === "test"),
          )
          .field(m.lax("env").default("dev"))
          .field(m.lax("laxField").default(0)),
      ).getModel();

      it("should respect default rules", async () => {
        const { data } = await Model.create({ isBlocked: true });

        expect(data).toMatchObject({
          env: "dev",
          isBlocked: true,
          laxField: 0,
        });
      });

      it("should respect callable should init when condition passes at creation", async () => {
        const { data } = await Model.create({
          env: "test",
          isBlocked: true,
        });

        expect(data).toEqual({
          env: "test",
          isBlocked: false,
          laxField: 0,
        });
      });

      describe("behaviour when ignoreInit method returns nothing", () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .lax("isBlocked")
                .default(false)
                .ignoreInit((() => {}) as never),
            )
            .field(m.lax("laxField").default(0)),
        ).getModel();

        it("should assume initialization as falsy if ignoreInit method returns nothing at creation", async () => {
          const { data } = await Model.create({ isBlocked: "yes" });

          expect(data).toMatchObject({ isBlocked: "yes", laxField: 0 });
        });
      });
    });

    describe("behaviour of callable ignoreInit", () => {
      const onSuccessValues: Record<string, unknown> = {};
      const onSuccessStats: Record<string, number> = {};
      const sanitizedValues: Record<string, unknown> = {};

      let Model: any;

      beforeAll(() => {
        Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .dependent("dependent", "virtual")
                .default("")
                .resolve(() => "changed")
                .onSuccess(onSuccess("dependent")),
            )
            .field(m.lax("laxField").default(""))
            .field(
              m
                .virtual("virtual")
                .validate(validateBoolean)
                .ignoreInit(
                  (input: any) => input?.laxField === "ignore virtual",
                )
                .sanitize(sanitizerOf("virtual", "sanitized"))
                .onSuccess([
                  onSuccess("virtual"),
                  incrementOnSuccessStats("virtual"),
                  incrementOnSuccessStats("virtual"),
                ]),
            ),
        ).getModel();

        function sanitizerOf(field: string, value: any) {
          return () => {
            // to make sure sanitizer is invoked
            sanitizedValues[field] = value;

            return value;
          };
        }

        function incrementOnSuccessStats(field: string) {
          return () => {
            onSuccessStats[field] = (onSuccessStats[field] ?? 0) + 1;
          };
        }

        type IgnoreInitInput = { virtual?: boolean; laxField?: string };
        type IgnoreInitOutput = { dependent: string; laxField: string };

        function onSuccess(field: string) {
          return ({
            input,
            values,
          }: ReadonlyIvoContext<IgnoreInitInput, IgnoreInitOutput>) => {
            onSuccessValues[field] =
              (values as Record<string, unknown>)?.[field] ??
              (input as Record<string, unknown>)?.[field];
            incrementOnSuccessStats(field)();
          };
        }

        function validateBoolean(value: any) {
          if (![false, true].includes(value))
            return { valid: false, reason: `${value} is not a boolean` };
          return { valid: true };
        }
      });

      beforeEach(() => {
        for (const key of Object.keys(onSuccessStats))
          delete onSuccessStats[key];
        for (const key of Object.keys(onSuccessValues))
          delete onSuccessValues[key];
        for (const key of Object.keys(sanitizedValues))
          delete sanitizedValues[key];
      });

      it("should respect virtuals at creation when their ignoreInit handler returns 'false'", async () => {
        const { data, handleSuccess } = await Model.create({
          laxField: "Peter",
          virtual: true,
        });

        await handleSuccess?.();

        expect(data).toEqual({ dependent: "changed", laxField: "Peter" });

        expect(onSuccessStats).toEqual({
          dependent: 1,
          virtual: 3,
        });

        expect(onSuccessValues).toEqual({
          dependent: "changed",
          virtual: "sanitized",
        });

        expect(sanitizedValues).toEqual({ virtual: "sanitized" });
      });

      it("should ignore virtuals at creation when their ignoreInit handler returns 'true'", async () => {
        const { data, handleSuccess } = await Model.create({
          laxField: "ignore virtual",
          virtual: true,
        });

        await handleSuccess?.();

        expect(data).toEqual({
          dependent: "",
          laxField: "ignore virtual",
        });

        expect(onSuccessStats).toEqual({ dependent: 1 });

        expect(onSuccessValues).toEqual({ dependent: "" });

        expect(sanitizedValues).toEqual({});
      });
    });
  });

  // "should reject ignoreInit(true) & no default" discarded: `.ignoreInit()`
  // isn't available on `LaxBuilder` until `.default()` has been called.
  describe("invalid", () => {
    it("should reject ignoreInit !(true | () => boolean)", () => {
      // `null` excluded: `.ignoreInit(resolver)` sets `resolver ?? true`,
      // so passing `null` is indistinguishable from calling `.ignoreInit()`
      // with no argument at all (both resolve to `true`, which is valid).
      const values = [false, 1, {}, [], "yes", "false", "true"];

      for (const ignoreInit of values) {
        const fxn = makeFx((b, m) =>
          b.field(
            m
              .lax("fieldName")
              .default(true)
              .ignoreInit(ignoreInit as never),
          ),
        );

        expectFailure(fxn);

        try {
          fxn();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                "The initialization of a property can only be blocked if the 'ignoreinit' rule is set to 'true' or a function that returns a boolean",
              ]),
            }),
          );
        }
      }
    });
  });
});

describe("ignoreUpdate", () => {
  describe("valid", () => {
    it("should accept ignoreUpdate(() => boolean)", () => {
      const validValues = [() => false, () => true];

      for (const ignoreUpdate of validValues) {
        const toPass = makeFx((b, m) =>
          b.field(m.lax("fieldName").default("").ignoreUpdate(ignoreUpdate)),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    it("should accept ignoreInit(() => boolean) & ignoreUpdate(false) for virtuals", () => {
      const values = [() => true, () => false];

      for (const ignoreInit of values) {
        const toPass = makeFx((b, m) =>
          b
            .field(
              m
                .dependent("dependentField", "virtual")
                .default("")
                .resolve(() => ""),
            )
            .field(
              m
                .virtual("virtual")
                .validate(validator)
                .ignoreInit(ignoreInit)
                .ignoreUpdate(),
            ),
        );

        expectNoFailure(toPass);

        toPass();
      }
    });

    describe("behaviour", () => {
      let onSuccessValues: Record<string, unknown> = {};
      let onSuccessStats: Record<string, number> = {};

      type IgnoreUpdateOutput = {
        dependentField: boolean;
        dependentField_1: boolean;
        laxField: string;
        laxField_1: string;
      };

      function incrementOnSuccessCountOf(field: string) {
        return ({
          input,
          values,
        }: ReadonlyIvoContext<
          {
            virtual: boolean;
            virtual_1: boolean;
            laxField: string;
            laxField_1: string;
          },
          IgnoreUpdateOutput
        >) => {
          const previousCount = onSuccessStats[field] ?? 0;

          onSuccessStats[field] = previousCount + 1;
          onSuccessValues[field] =
            (values as Record<string, unknown>)[field] ??
            (input as Record<string, unknown>)?.[field];
        };
      }

      const Model = new Schema<any>((b, m) =>
        b
          .field(
            m
              .dependent("dependentField", "virtual")
              .default(false)
              .resolve(({ input }: any) => input.virtual)
              .onSuccess(incrementOnSuccessCountOf("dependentField")),
          )
          .field(
            m
              .dependent("dependentField_1", "virtual_1")
              .default(false)
              .resolve(({ input }: any) => input.virtual_1)
              .onSuccess(incrementOnSuccessCountOf("dependentField_1")),
          )
          .field(
            m
              .lax("laxField")
              .default("")
              .readonly()
              .ignoreUpdate(
                (_input: any, previousValues: any) =>
                  previousValues?.laxField_1 === "test",
              )
              .onSuccess(incrementOnSuccessCountOf("laxField")),
          )
          .field(m.lax("laxField_1").default("dev"))
          .field(
            m
              .virtual("virtual")
              .validate(() => ({ valid: true }))
              .ignoreUpdate()
              .onSuccess(incrementOnSuccessCountOf("virtual")),
          )
          .field(
            m
              .virtual("virtual_1")
              .validate(() => ({ valid: true }))
              .ignoreUpdate(
                (_input: any, previousValues: any) =>
                  previousValues?.laxField_1 === "test",
              )
              .onSuccess(incrementOnSuccessCountOf("virtual_1")),
          ),
      ).getModel();

      afterEach(() => {
        onSuccessValues = {};
        onSuccessStats = {};
      });

      it("should update properties when 'ignoreUpdate' resolved to 'false'", async () => {
        const { data, error, handleSuccess } = await Model.update(
          {
            dependentField: "dev",
            dependentField_1: "dev",
            laxField: "",
            laxField_1: "",
          },
          { laxField: "yoyo", virtual: true, virtual_1: true },
        );

        await handleSuccess?.();

        expect(error).toBeNull();
        expect(data).toEqual({ dependentField_1: true, laxField: "yoyo" });

        expect(onSuccessStats).toEqual({
          dependentField_1: 1,
          laxField: 1,
          virtual_1: 1,
        });

        expect(onSuccessValues).toEqual({
          dependentField_1: true,
          laxField: "yoyo",
          virtual_1: true,
        });
      });

      it("should not update properties when 'ignoreUpdate' resolved to 'true'", async () => {
        const { data, error } = await Model.update(
          {
            dependentField: "dev",
            dependentField_1: "dev",
            laxField: "",
            laxField_1: "test",
          },
          { laxField: "yoyo", virtual: true, virtual_1: true },
        );

        expect(data).toBeNull();
        expect(error).toBeNull();
      });

      it("should not update readonly properties that have changed even when 'ignoreUpdate' resolved to 'false'", async () => {
        const { data, error } = await Model.update(
          {
            dependentField: "dev",
            dependentField_1: "dev",
            laxField: "changed",
            laxField_1: "test",
          },
          { laxField: "yoyo" },
        );

        expect(data).toBeNull();
        expect(error).toBeNull();
      });

      describe("behaviour when ignoreUpdate method returns nothing", () => {
        const Model = new Schema<any>((b, m) =>
          b
            .field(
              m
                .lax("isBlocked")
                .default(false)
                .ignoreUpdate((() => {}) as never),
            )
            .field(m.lax("laxField").default(0)),
        ).getModel();

        it("should update property if ignoreUpdate method returns nothing", async () => {
          const { data, error } = await Model.update(
            { isBlocked: false, laxField: 0 },
            { isBlocked: true },
          );

          expect(error).toBeNull();
          expect(data).toEqual({ isBlocked: true });
        });
      });
    });
  });

  describe("invalid", () => {
    it("should reject ignoreUpdate !(false | () => boolean)", () => {
      // `null` excluded: `.ignoreUpdate(resolver)` sets `resolver ?? true`,
      // so passing `null` is indistinguishable from calling `.ignoreUpdate()`
      // with no argument at all (both resolve to `true`, which is valid).
      const invalidValues = [1, 0, -1, "true", "false", [], {}];

      for (const ignoreUpdate of invalidValues) {
        const toFail = makeFx((b, m) =>
          b.field(
            m
              .lax("fieldName")
              .default("")
              .ignoreUpdate(ignoreUpdate as never),
          ),
        );

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              fieldName: expect.arrayContaining([
                "'ignoreUpdate' only accepts true or a function that returns a boolean",
              ]),
            }),
          );
        }
      }
    });

    it("should allow readonly(true) + ignoreUpdate(function) (ignoreUpdate just needs to not be literal `true`)", () => {
      const toPass = makeFx((b, m) =>
        b.field(
          m
            .lax("fieldName")
            .default("")
            .readonly()
            .ignoreUpdate(() => true),
        ),
      );

      expectNoFailure(toPass);

      toPass();
    });

    it("should reject readonly(true) + ignoreUpdate(true) ('use a function for ignoreUpdate instead')", () => {
      const toFail = makeFx((b, m) =>
        b.field(m.lax("fieldName").default("").readonly().ignoreUpdate()),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            fieldName: expect.arrayContaining([
              "Both 'readonly' & 'ignoreUpdate' cannot be 'true'. Use a function for 'ignoreUpdate' instead",
            ]),
          }),
        );
      }
    });
  });
});

describe("ignoreInit & ignoreUpdate", () => {
  describe("valid", () => {
    it("should accept ignoreInit & ignoreUpdate for lax props", () => {
      // [ignoreInit, ignoreUpdate]
      const values = [
        [() => {}, () => {}],
        [true, () => {}],
        [() => {}, true],
      ];

      for (const [ignoreInit, ignoreUpdate] of values) {
        const toPass = makeFx((b, m) => {
          let builder: any = m.lax("fieldName").default("");
          builder =
            ignoreInit === true
              ? builder.ignoreInit()
              : builder.ignoreInit(ignoreInit as never);
          builder =
            ignoreUpdate === true
              ? builder.ignoreUpdate()
              : builder.ignoreUpdate(ignoreUpdate as never);

          return b.field(builder);
        });

        expectNoFailure(toPass);

        toPass();
      }
    });

    it("should accept ignoreInit(() => boolean) + ignoreUpdate(true | () => boolean) + readonly(true)", () => {
      // [ignoreInit, ignoreUpdate]
      const readonlyTrue = [
        [() => {}, () => {}],
        [true, () => {}],
      ];

      for (const [ignoreInit, ignoreUpdate] of readonlyTrue) {
        const toPass = makeFx((b, m) => {
          let builder: any = m
            .lax("dependentField")
            .default("")
            .validate(validator)
            .readonly();
          builder =
            ignoreInit === true
              ? builder.ignoreInit()
              : builder.ignoreInit(ignoreInit as never);
          builder =
            ignoreUpdate === true
              ? builder.ignoreUpdate()
              : builder.ignoreUpdate(ignoreUpdate as never);

          return b.field(builder);
        });

        expectNoFailure(toPass);

        toPass();
      }
    });
  });

  describe("invalid", () => {
    it("should reject ignoreUpdate == true & ignoreInit == true", () => {
      const toFail = makeFx((b, m) =>
        b.field(m.lax("fieldName").default("").ignoreInit().ignoreUpdate()),
      );

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            fieldName: expect.arrayContaining([
              "Both 'ignoreInit' & 'ignoreUpdate' cannot be 'true'",
            ]),
          }),
        );
      }
    });
  });
});
