import {
  expectFailure,
  expectNoFailure,
  expectPromiseFailure,
  getValidSchema,
} from "../_utils";

export const Test_SchemaErrors = ({ Schema, fx }: any) => {
  describe("errors", () => {
    it("should allow 'silent' | 'throw'", () => {
      const values = ["silent", "throw"];

      for (const errors of values) {
        const toPass = fx(getValidSchema(), { errors });

        expectNoFailure(toPass);

        toPass();
      }
    });

    describe("valid", () => {
      let silentModel: any,
        modelToThrow: any,
        models: any[] = [];

      beforeAll(() => {
        const validator = (value: any) => {
          return value
            ? { valid: true }
            : { reason: "Invalid value", valid: false };
        };

        const definition = {
          lax: { default: "lax-default", validator },
          readonly: {
            readonly: "lax",
            default: "readonly-default",
            validator,
          },
          required: { required: true, validator },
        };

        silentModel = new Schema(definition).getModel();
        modelToThrow = new Schema(definition, { errors: "throw" }).getModel();

        models = [silentModel, modelToThrow];
      });

      describe("silent & throw with valid data", () => {
        for (const model of models) {
          // create
          it("should create normally", async () => {
            const { data } = await model.create({
              readonly: "lax",
              required: true,
            });

            expect(data).toEqual({
              lax: "lax-default",
              readonly: "lax",
              required: true,
            });
          });

          // clone
          it("should clone normally", async () => {
            const { data } = await model.clone({
              readonly: "lax",
              required: true,
            });

            expect(data).toEqual({
              lax: "lax-default",
              readonly: "lax",
              required: true,
            });
          });

          // update
          it("should update normally", async () => {
            const { data } = await model.update(
              {
                lax: "lax-default",
                readonly: "lax",
                required: true,
              },
              { required: "required" }
            );

            expect(data).toEqual({ required: "required" });
          });
        }
      });

      describe("silent", () => {
        // create
        it("should reject invalid props on create", async () => {
          const { error } = await silentModel.create({
            lax: false,
            readonly: "lax",
            required: "",
          });

          expect(error).toEqual(
            expect.objectContaining({
              message: "Validation Error",
              payload: {
                lax: ["Invalid value"],
                required: ["Invalid value"],
              },
              statusCode: 400,
            })
          );
        });

        // clone
        it("should reject invalid props on clone", async () => {
          const { error } = await silentModel.clone({
            lax: false,
            readonly: "lax",
            required: "",
          });

          expect(error).toEqual(
            expect.objectContaining({
              message: "Validation Error",
              payload: {
                lax: ["Invalid value"],
                required: ["Invalid value"],
              },
              statusCode: 400,
            })
          );
        });

        // update
        it("should reject invalid props on update", async () => {
          const { error } = await silentModel.update(
            {
              lax: "lax-default",
              readonly: "lax",
              required: true,
            },
            { lax: false, required: "" }
          );

          expect(error).toEqual(
            expect.objectContaining({
              message: "Validation Error",
              payload: {
                lax: ["Invalid value"],
                required: ["Invalid value"],
              },
              statusCode: 400,
            })
          );
        });

        it("should reject on nothing to update", async () => {
          const { error } = await silentModel.update(
            {
              lax: "lax-default",
              readonly: "lax",
              required: true,
            },
            { readonly: "New val" }
          );

          expect(error).toEqual(
            expect.objectContaining({
              message: "Nothing to update",
              payload: {},
              statusCode: 400,
            })
          );
        });
      });

      describe("throw", () => {
        // create
        it("should reject invalid props on create", async () => {
          const toFail = () =>
            modelToThrow.create({
              lax: false,
              readonly: "lax",
              required: "",
            });

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: "Validation Error",
                payload: {
                  lax: ["Invalid value"],
                  required: ["Invalid value"],
                },
                statusCode: 400,
              })
            );
          }
        });

        // clone
        it("should reject invalid props on clone", async () => {
          const toFail = () =>
            modelToThrow.clone({
              lax: false,
              readonly: "lax",
              required: "",
            });

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: "Validation Error",
                payload: {
                  lax: ["Invalid value"],
                  required: ["Invalid value"],
                },
                statusCode: 400,
              })
            );
          }
        });

        // update
        it("should reject invalid props on update", async () => {
          const toFail = () =>
            modelToThrow.update(
              {
                lax: "lax-default",
                readonly: "lax",
                required: true,
              },
              { lax: false, required: "" }
            );

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: "Validation Error",
                payload: {
                  lax: ["Invalid value"],
                  required: ["Invalid value"],
                },
                statusCode: 400,
              })
            );
          }
        });

        it("should reject on nothing to update", async () => {
          const toFail = () =>
            modelToThrow.update(
              {
                lax: "lax-default",
                readonly: "lax",
                required: true,
              },
              { readonly: "New val" }
            );

          expectPromiseFailure(toFail, "Nothing to update");

          try {
            await toFail();
          } catch (err: any) {
            expect(err).toEqual(
              expect.objectContaining({
                message: "Nothing to update",
                payload: {},
                statusCode: 400,
              })
            );
          }
        });
      });
    });

    describe("invalid", () => {
      it("should reject anything other than ('silent' | 'throw')", () => {
        const values = ["silence", 1, null, false, true, "throws", [], {}];

        for (const errors of values) {
          const toFail = fx(getValidSchema(), { errors });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                errors: expect.arrayContaining([
                  "should be 'silent' or 'throw'",
                ]),
              })
            );
          }
        }
      });
    });
  });
};
