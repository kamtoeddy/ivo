import { describe, it, expect } from "bun:test";

import { expectFailure, expectNoFailure, validator } from "../_utils";

export const Test_AllowedValues = ({ fx, Schema }: any) => {
  describe("allowed values", () => {
    describe("valid", () => {
      it("should not reject if allowed values provided are >= 2", () => {
        const values = [
          ["lol", 2],
          ["lol", 2, 3],
        ];

        for (const allow of values) {
          const toPass = fx({ prop: { default: allow[0], allow } });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it("should not reject if default value provided is an allowed value", () => {
        const toPass = fx({
          prop: { default: null, allow: [null, "lolz", -1] },
        });

        expectNoFailure(toPass);

        toPass();
      });

      it("should allow virtuals to have allowed values", () => {
        const toPass = fx({
          dependent: {
            default: true,
            dependsOn: "virtual",
            resolver: validator,
          },
          virtual: { virtual: true, allow: [null, "lolz", -1], validator },
        });

        expectNoFailure(toPass);

        toPass();
      });

      describe("allow as an object", () => {
        it('should not reject if "values" is the only key provided', () => {
          const toPass = fx({
            dependent: {
              default: null,
              allow: { values: [null, "lolz", -1] },
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it('should not reject if "values" & "error" are both provided', () => {
          const toPass = fx({
            dependent: {
              default: null,
              allow: { error: "value not allowed", values: [null, "lolz", -1] },
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should not reject if validator is provided", () => {
          const toPass = fx({
            dependent: {
              default: null,
              allow: { error: "value not allowed", values: [null, "lolz", -1] },
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should not reject with errors valid formats", () => {
          const errors = [
            "value not allowed",
            { reason: "invalid value" },
            { reason: "invalid value", metadata: {} },
            { metadata: {} },
            () => "",
          ];

          for (const error of errors) {
            const toPass = fx({
              dependent: {
                default: null,
                allow: { error, values: [null, "lolz", -1] },
                validator,
              },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });
      });
    });

    describe("invalid", () => {
      it("should reject if non-array value is provided", () => {
        const values = [
          null,
          undefined,
          new Number(),
          new String(),
          Symbol(),
          2,
          -10,
          true,
          () => {},
          {},
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ["Allowed values must be an array"],
            });
          }
        }
      });

      it("should reject if allowed values provided are not unique", () => {
        const values = [
          [1, 2, 2, 4, 5],
          ["lol", 59, "lol", null],
          [true, false, true],
          [{}, {}],
          [{ id: "lol" }, { id: "lol" }],
        ];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ["Allowed values must be an array of unique values"],
            });
          }
        }
      });

      it("should reject if allowed values provided are less than 2", () => {
        const values = [[], ["lol"]];

        for (const allow of values) {
          const toFail = fx({ prop: { allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ["Allowed values must have at least 2 values"],
            });
          }
        }
      });

      it("should reject if default value provided is not an allowed value", () => {
        const values = [
          ["lol", [null, "lolz", -1]],
          [null, [1, 4, "lol", undefined]],
        ];

        for (const [_default, allow] of values) {
          const toFail = fx({ prop: { default: _default, allow } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ["The default value must be an allowed value"],
            });
          }
        }
      });

      describe("allow as an object", () => {
        it("should reject if values array is not provided", () => {
          const toFail = fx({ prop: { allow: {} } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: ["Allowed values must be an array"],
            });
          }
        });

        it("should reject if non-array value is provided", () => {
          const invalidValues = [
            null,
            undefined,
            new Number(),
            new String(),
            Symbol(),
            2,
            -10,
            true,
            () => {},
            {},
          ];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ["Allowed values must be an array"],
              });
            }
          }
        });

        it("should reject if allowed values provided are not unique", () => {
          const invalidValues = [
            [1, 2, 2, 4, 5],
            ["lol", 59, "lol", null],
            [true, false, true],
            [{}, {}],
            [{ id: "lol" }, { id: "lol" }],
          ];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ["Allowed values must be an array of unique values"],
              });
            }
          }
        });

        it("should reject if allowed values provided are less than 2", () => {
          const invalidValues = [[], ["lol"]];

          for (const values of invalidValues) {
            const toFail = fx({ prop: { allow: { values } } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ["Allowed values must have at least 2 values"],
              });
            }
          }
        });

        it("should reject if default value provided is not an allowed value", () => {
          const data = [
            ["lol", [null, "lolz", -1]],
            [null, [1, 4, "lol", undefined]],
          ];

          for (const [_default, values] of data) {
            const toFail = fx({
              prop: { default: _default, allow: { values } },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: ["The default value must be an allowed value"],
              });
            }
          }
        });

        it("should reject if error is provided and is of invalid type", () => {
          const errors = [
            null,
            true,
            false,
            {},
            { key: "value" },
            -1,
            0,
            1,
            [],
            [[], null, true, false, {}, { key: "value" }, -1, 0, 1],
          ];

          for (const error of errors) {
            const toFail = fx({
              prop: {
                default: null,
                allow: { error, values: [null, "lolz", -1] },
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject({
                prop: [
                  'The "error" field of the allow rule can only accept a string, InputFieldError or an function that returns any of the above mentioned',
                ],
              });
            }
          }
        });

        it("should reject if an invalid config key is passed", () => {
          const toFail = fx({
            prop: {
              default: null,
              allow: { key: "value", values: [null, "lolz", -1] },
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject({
              prop: [
                'The "allow" rule only accepts "error" & "values" as configuration. Please remove the extra keys',
              ],
            });
          }
        });
      });
    });

    describe("behaviour", () => {
      const metadata = { allowed: [null, "allowed"] };

      describe("behaviour with lax props & no validators", () => {
        const Model = new Schema({
          prop: { default: null, allow: metadata.allowed },
        }).getModel();

        describe("creation", () => {
          it("should allow if value provided is allowed", async () => {
            const { data, error } = await Model.create({ prop: "allowed" });

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: "allowed" });
          });

          it("should reject if value provided is not allowed", async () => {
            const { data, error } = await Model.create({ prop: true });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "value not allowed",
                metadata,
              }),
            });
          });
        });

        describe("updates", () => {
          it("should allow if value provided is allowed", async () => {
            const { data, error } = await Model.update(
              { prop: "allowed" },
              { prop: null },
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: null });
          });

          it("should reject if value provided is not allowed", async () => {
            const { data, error } = await Model.update(
              { prop: null },
              { prop: true },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "value not allowed",
                metadata,
              }),
            });
          });
        });
      });

      describe("behaviour with lax props & validators", () => {
        const Model = new Schema({
          prop: {
            default: null,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: "validated" };

              return false;
            },
          },
        }).getModel();

        describe("creation", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.create({ prop: null });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.create({ prop: "allowed" });

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: "allowed" });
          });
        });

        describe("updates", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.update(
              { prop: "allowed" },
              { prop: null },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.update(
              { prop: null },
              { prop: "allowed" },
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: "allowed" });
          });
        });
      });

      describe("behaviour with required props & no validators", () => {
        const Model = new Schema({
          prop: { required: true, allow: metadata.allowed },
        }).getModel();

        describe("creation", () => {
          it("should accept allowed values if provided", async () => {
            const { data, error } = await Model.create({ prop: null });

            expect(error).toBeNull();
            expect(data).toEqual({ prop: null });
          });

          it("should reject non-allowed values if provided", async () => {
            const { data, error } = await Model.create({ prop: "lolz" });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "value not allowed",
                metadata,
              }),
            });
          });

          it("should reject if no value is provided", async () => {
            const { data, error } = await Model.create();

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "value not allowed",
                metadata,
              }),
            });
          });
        });

        describe("updates", () => {
          it("should accept allowed values if provided", async () => {
            const { data, error } = await Model.update(
              { prop: "allowed" },
              { prop: null },
            );

            expect(error).toBeNull();
            expect(data).toEqual({ prop: null });
          });

          it("should reject non-allowed values if provided", async () => {
            const { data, error } = await Model.update(
              { prop: "allowed" },
              { prop: "whatever" },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "value not allowed",
                metadata,
              }),
            });
          });
        });
      });

      describe("behaviour with required props & validators", () => {
        const Model = new Schema({
          prop: {
            required: true,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: "validated" };

              return false;
            },
          },
        }).getModel();

        describe("creation", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.create({ prop: null });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.create({ prop: "allowed" });

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: "allowed" });
          });
        });

        describe("updates", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.update(
              { prop: "allowed" },
              { prop: null },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              prop: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.update(
              { prop: null },
              { prop: "allowed" },
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ prop: "allowed" });
          });
        });
      });

      describe("behaviour with virtuals", () => {
        const Model = new Schema({
          dependent: {
            default: null,
            dependsOn: "virtual",
            resolver: ({ context: { virtual } }) => virtual,
          },
          virtual: {
            virtual: true,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: "validated" };

              return false;
            },
          },
        }).getModel();

        describe("creation", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.create({ virtual: null });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              virtual: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.create({ virtual: "allowed" });

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: "allowed" });
          });
        });

        describe("updates", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.update(
              { dependent: "allowed" },
              { virtual: null },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              virtual: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.update(
              { dependent: null },
              { virtual: "allowed" },
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: "allowed" });
          });
        });
      });

      describe("behaviour with virtuals & alias", () => {
        const Model = new Schema({
          dependent: {
            default: null,
            dependsOn: "virtual",
            resolver: ({ context: { virtual } }) => virtual,
          },
          virtual: {
            alias: "dependent",
            virtual: true,
            allow: metadata.allowed,
            validator(v: any) {
              if (v) return { valid: true, validated: "validated" };

              return false;
            },
          },
        }).getModel();

        describe("creation", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.create({ dependent: null });

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              dependent: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.create({
              dependent: "allowed",
            });

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: "allowed" });
          });
        });

        describe("updates", () => {
          it("should respect validators even if value provided is allowed", async () => {
            const { data, error } = await Model.update(
              { dependent: "allowed" },
              { dependent: null },
            );

            expect(data).toBeNull();
            expect(error.payload).toMatchObject({
              dependent: expect.objectContaining({
                reason: "validation failed",
              }),
            });
          });

          it("should ignore validated value from validator if value is not allowed", async () => {
            const { data, error } = await Model.update(
              { dependent: null },
              { dependent: "allowed" },
            );

            expect(error).toBeNull();
            expect(data).toMatchObject({ dependent: "allowed" });
          });
        });
      });

      describe("allow as an object", () => {
        describe("behaviour with lax props & no validators", () => {
          const Model = new Schema({
            prop: { default: null, allow: { values: metadata.allowed } },
          }).getModel();

          describe("creation", () => {
            it("should allow if value provided is allowed", async () => {
              const { data, error } = await Model.create({ prop: "allowed" });

              expect(error).toBeNull();
              expect(data).toMatchObject({ prop: "allowed" });
            });

            it("should reject if value provided is not allowed", async () => {
              const { data, error } = await Model.create({ prop: true });

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                prop: expect.objectContaining({
                  reason: "value not allowed",
                  metadata,
                }),
              });
            });
          });

          describe("updates", () => {
            it("should allow if value provided is allowed", async () => {
              const { data, error } = await Model.update(
                { prop: "allowed" },
                { prop: null },
              );

              expect(error).toBeNull();
              expect(data).toMatchObject({ prop: null });
            });

            it("should reject if value provided is not allowed", async () => {
              const { data, error } = await Model.update(
                { prop: null },
                { prop: true },
              );

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                prop: expect.objectContaining({
                  reason: "value not allowed",
                  metadata,
                }),
              });
            });
          });
        });

        describe("behaviour with lax props & validators", () => {
          const Model = new Schema({
            prop: {
              default: null,
              allow: { values: metadata.allowed },
              validator(v: any) {
                if (v) return { valid: true, validated: "validated" };

                return false;
              },
            },
          }).getModel();

          describe("creation", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.create({ prop: null });

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                prop: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.create({ prop: "allowed" });

              expect(error).toBeNull();
              expect(data).toMatchObject({ prop: "allowed" });
            });
          });

          describe("updates", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.update(
                { prop: "allowed" },
                { prop: null },
              );

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                prop: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.update(
                { prop: null },
                { prop: "allowed" },
              );

              expect(error).toBeNull();
              expect(data).toMatchObject({ prop: "allowed" });
            });
          });
        });

        describe("behaviour with virtuals", () => {
          const Model = new Schema({
            dependent: {
              default: null,
              dependsOn: "virtual",
              resolver: ({ context: { virtual } }) => virtual,
            },
            virtual: {
              virtual: true,
              allow: { values: metadata.allowed },
              validator(v: any) {
                if (v) return { valid: true, validated: "validated" };

                return false;
              },
            },
          }).getModel();

          describe("creation", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.create({ virtual: null });

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                virtual: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.create({
                virtual: "allowed",
              });

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: "allowed" });
            });
          });

          describe("updates", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.update(
                { dependent: "allowed" },
                { virtual: null },
              );

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                virtual: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.update(
                { dependent: null },
                { virtual: "allowed" },
              );

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: "allowed" });
            });
          });
        });

        describe("behaviour with virtuals & alias", () => {
          const Model = new Schema({
            dependent: {
              default: null,
              dependsOn: "virtual",
              resolver: ({ context: { virtual } }) => virtual,
            },
            virtual: {
              alias: "dependent",
              virtual: true,
              allow: { values: metadata.allowed },
              validator(v: any) {
                if (v) return { valid: true, validated: "validated" };

                return false;
              },
            },
          }).getModel();

          describe("creation", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.create({ dependent: null });

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                dependent: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.create({
                dependent: "allowed",
              });

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: "allowed" });
            });
          });

          describe("updates", () => {
            it("should respect validators even if value provided is allowed", async () => {
              const { data, error } = await Model.update(
                { dependent: "allowed" },
                { dependent: null },
              );

              expect(data).toBeNull();
              expect(error.payload).toMatchObject({
                dependent: expect.objectContaining({
                  reason: "validation failed",
                }),
              });
            });

            it("should ignore validated value from validator if value is not allowed", async () => {
              const { data, error } = await Model.update(
                { dependent: null },
                { dependent: "allowed" },
              );

              expect(error).toBeNull();
              expect(data).toMatchObject({ dependent: "allowed" });
            });
          });
        });

        describe("error", () => {
          describe("error as a string", () => {
            describe("if string is empty", () => {
              const Model = new Schema({
                prop: {
                  default: metadata.allowed[0],
                  allow: { error: "", values: metadata.allowed },
                },
              }).getModel();

              it("should return default error message at creation", async () => {
                const { data, error } = await Model.create({ prop: "Invalid" });

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({
                    reason: "value not allowed",
                  }),
                });
              });

              it("should return default error message during updates", async () => {
                const { data, error } = await Model.update(
                  { prop: metadata.allowed[0] },
                  { prop: "Invalid" },
                );

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({
                    reason: "value not allowed",
                  }),
                });
              });
            });

            describe("if string is not empty", () => {
              const errorMessage = "Value not allowed. lol";

              const Model = new Schema({
                prop: {
                  default: metadata.allowed[0],
                  allow: { error: errorMessage, values: metadata.allowed },
                },
              }).getModel();

              it("should return default error message at creation", async () => {
                const { data, error } = await Model.create({ prop: "Invalid" });

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({ reason: errorMessage }),
                });
              });

              it("should return default error message during updates", async () => {
                const { data, error } = await Model.update(
                  { prop: metadata.allowed[0] },
                  { prop: "Invalid" },
                );

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({ reason: errorMessage }),
                });
              });
            });
          });

          describe("error as InputFieldError", () => {
            const errorMessages = [
              [{ reason: "Invalid lol" }],
              [
                {
                  reason: "failed again",
                  metadata: { allowed: metadata.allowed },
                },
              ],
            ];

            for (const [expected] of errorMessages) {
              const Model = new Schema({
                prop: {
                  default: metadata.allowed[0],
                  allow: { error: expected, values: metadata.allowed },
                },
              }).getModel();

              it("should return default error message at creation", async () => {
                const { data, error } = await Model.create({ prop: "Invalid" });

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining(expected),
                });
              });

              it("should return default error message during updates", async () => {
                const { data, error } = await Model.update(
                  { prop: metadata.allowed[0] },
                  { prop: "Invalid" },
                );

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining(expected),
                });
              });
            }
          });

          describe("error as function", () => {
            const reason = "value not allowed";
            const errorMessages = [
              [() => "", { reason }],
              [() => ({}), { reason }],
              [() => null, { reason }],
              [() => undefined, { reason }],
              [() => -1, { reason }],
              [() => 0, { reason }],
              [() => 1, { reason }],
              [() => true, { reason }],
              [() => false, { reason }],
              [() => "Invalid lol", { reason: "Invalid lol" }],
              [() => ["invalid as array", "Invalid lol"], { reason }],
              [
                () => ({ metadata: { valid: false } }),
                { metadata: { valid: false } },
              ],
              [() => ({ reason: "Invalid lol" }), { reason: "Invalid lol" }],
              [
                () => ({
                  reason: "failed again",
                  metadata: { allowed: metadata.allowed },
                }),
                {
                  reason: "failed again",
                  metadata: { allowed: metadata.allowed },
                },
              ],
            ];

            for (const [error, expected] of errorMessages) {
              const Model = new Schema({
                prop: {
                  default: metadata.allowed[0],
                  allow: { error, values: metadata.allowed },
                },
              }).getModel();

              it("should return default error message at creation", async () => {
                const { data, error } = await Model.create({ prop: "Invalid" });

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining(expected),
                });
              });

              it("should return default error message during updates", async () => {
                const { data, error } = await Model.update(
                  { prop: metadata.allowed[0] },
                  { prop: "Invalid" },
                );

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining(expected),
                });
              });
            }

            describe("behaviour with errors thrown in the error setter", () => {
              const Model = new Schema({
                prop: {
                  default: "lol",
                  allow: {
                    values: ["lol", "lolol"],
                    error() {
                      throw new Error("lolol");
                    },
                  },
                },
              }).getModel();

              it("should return proper errors at creation", async () => {
                const { data, error } = await Model.create({ prop: "" });

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({
                    reason: "value not allowed",
                  }),
                });
              });

              it("should return proper errors during updates", async () => {
                const { data, error } = await Model.update(
                  { prop: "lol" },
                  { prop: "" },
                );

                expect(data).toBeNull();
                expect(error.payload).toMatchObject({
                  prop: expect.objectContaining({
                    reason: "value not allowed",
                  }),
                });
              });
            });
          });
        });
      });
    });
  });
};
