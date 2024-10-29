import { describe, it, expect, beforeEach } from "bun:test";

import { ERRORS } from "../../../dist";
import { expectFailure, expectNoFailure, validator } from "../_utils";

export const Test_RequiredProperties = ({ Schema, fx }: any) => {
  describe("required", () => {
    describe("valid", () => {
      it("should allow required + validator", () => {
        const toPass = fx({ propertyName: { required: true, validator } });

        expectNoFailure(toPass);

        toPass();
      });

      it("should allow required: true + allow alone", () => {
        const toPass = fx({
          propertyName: { required: true, allow: [1, 2, 435, 45] },
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe("invalid", () => {
      it("should reject required & no validator", () => {
        const toFail = fx({ propertyName: { required: true } });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Required properties must have a validator",
              ]),
            }),
          );
        }
      });

      it("should reject required(true) + default", () => {
        const toFail = fx({
          propertyName: { default: "", required: true, validator },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Strictly required properties cannot have a default value or setter",
              ]),
            }),
          );
        }
      });

      it("should reject required(true) + readonly(true)", () => {
        const values = [false, true];

        for (const readonly of values) {
          const toFail = fx({
            propertyName: { readonly, required: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Strictly required properties cannot be readonly",
                ]),
              }),
            );
          }
        }
      });

      it("should reject required(true) + shouldInit", () => {
        const values = [false, true, [], {}];

        for (const shouldInit of values) {
          const toFail = fx({
            propertyName: { required: true, shouldInit, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Strictly Required properties cannot have a initialization blocked",
                ]),
              }),
            );
          }
        }
      });
    });
  });

  describe("requiredBy", () => {
    describe("behaviour", () => {
      let callsPerProp = {} as never;

      const book = {
        bookId: 1,
        isPublished: false,
        price: null,
        priceReadonly: null,
        priceRequiredWithoutMessage: null,
      };

      function validatePrice(price: any) {
        const validated = Number(price),
          valid = !isNaN(price) && validated;
        return { valid, validated };
      }

      function recordCalls(prop: string) {
        callsPerProp[prop] = true;
      }

      const Book = new Schema({
        bookId: { required: true, validator },
        isPublished: { default: false, validator },
        price: {
          default: null,
          required({ context: { isPublished, price } }: any) {
            const isRequired = isPublished && price == null;
            recordCalls("price");
            return [isRequired, "A price is required to publish a book!"];
          },
          validator: validatePrice,
        },
        priceReadonly: {
          default: null,
          readonly: true,
          required({ context: { price, priceReadonly } }: any) {
            const isRequired = price == 101 && priceReadonly == null;
            recordCalls("priceReadonly");
            return [
              isRequired,
              "A priceReadonly is required when price is 101!",
            ];
          },
          validator: validatePrice,
        },
        priceRequiredWithoutMessage: {
          default: null,
          readonly: true,
          required: ({ context: { price, priceReadonly } }: any) => {
            recordCalls("priceRequiredWithoutMessage");
            return price == 101 && priceReadonly == null;
          },
          validator: validatePrice,
        },
      }).getModel();

      beforeEach(() => {
        callsPerProp = {};
      });

      describe("creation", () => {
        it("should create normally", async () => {
          const toPass = () => Book.create({ bookId: 1 });

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual(book);
          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });

        it("should pass if condition is met at creation", async () => {
          const toPass = () =>
            Book.create({ bookId: 1, isPublished: true, price: 2000 });

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({
            bookId: 1,
            isPublished: true,
            price: 2000,
            priceReadonly: null,
            priceRequiredWithoutMessage: null,
          });
          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });

        it("should reject if condition is not met at creation", async () => {
          const { data, error } = await Book.create({
            bookId: 1,
            isPublished: true,
          });

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              price: {
                reason: "A price is required to publish a book!",
                metadata: null,
              },
            },
          });

          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });
      });

      describe("updates", () => {
        it("should pass if condition is met during updates", async () => {
          const toPass = () =>
            Book.update(
              {
                bookId: 1,
                isPublished: false,
                price: null,
                priceReadonly: null,
                priceRequiredWithoutMessage: null,
              },
              { isPublished: true, price: 20 },
            );

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({ isPublished: true, price: 20 });
          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });

        it("should pass if condition is met during updates of readonly", async () => {
          const toPass = () =>
            Book.update(book, { price: 101, priceReadonly: 201 });

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({ price: 101, priceReadonly: 201 });
          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });

        it("should reject if condition is not met during updates", async () => {
          const { data, error } = await Book.update(
            {
              bookId: 1,
              isPublished: false,
              price: null,
              priceReadonly: null,
            },
            { isPublished: true },
          );

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              price: {
                reason: "A price is required to publish a book!",
                metadata: null,
              },
            },
          });

          expect(callsPerProp).toEqual({ price: true, priceReadonly: true });
        });

        it("should reject if condition is not met during updates of readonly", async () => {
          const { data, error } = await Book.update(book, { price: 101 });

          expect(data).toBeNull();
          expect(error).toMatchObject({
            message: ERRORS.VALIDATION_ERROR,
            payload: {
              priceReadonly: {
                reason: "A priceReadonly is required when price is 101!",
                metadata: null,
              },
              priceRequiredWithoutMessage: {
                reason: "'priceRequiredWithoutMessage' is required",
                metadata: null,
              },
            },
          });

          expect(callsPerProp).toEqual({
            price: true,
            priceReadonly: true,
            priceRequiredWithoutMessage: true,
          });
        });

        it("should not update callable readonly prop that has changed", async () => {
          const { data, error } = await Book.update(
            {
              bookId: 1,
              isPublished: false,
              price: null,
              priceReadonly: 3000,
              priceRequiredWithoutMessage: null,
            },
            { priceReadonly: 101, priceRequiredWithoutMessage: 2000 },
          );

          expect(error).toBeNull();
          expect(data).toEqual({ priceRequiredWithoutMessage: 2000 });
          expect(callsPerProp).toEqual({
            price: true,
            priceRequiredWithoutMessage: true,
          });
        });
      });

      describe("behaviour when nothing is returned from required function", () => {
        const Book = new Schema({
          bookId: { required: true, validator },
          isPublished: { default: false, validator },
          name: { default: "", validator },
          price: {
            default: null,
            required() {},
            validator: validator,
          },
        }).getModel();

        it("should create normally", async () => {
          const { data } = await Book.create({ bookId: 1 });

          expect(data).toEqual({
            bookId: 1,
            isPublished: false,
            name: "",
            price: null,
          });
        });

        it("should update normally", async () => {
          const book = {
            bookId: 1,
            isPublished: false,
            name: "",
            price: null,
          };
          const { data } = await Book.update(book, { name: "yooo" });

          expect(data).toEqual({ name: "yooo" });
        });
      });

      describe("behaviour when a non-string value is returned as message from required function", () => {
        describe("should respect InputField", () => {
          const responses = [
            [{ reason: "lol" }, { reason: "lol" }],
            [
              { reason: "lol", metadata: { shouldWork: true } },
              { reason: "lol", metadata: { shouldWork: true } },
            ],
            [
              { reason: "", metadata: null },
              { reason: "'price' is required", metadata: null },
            ],
            [
              { metadata: null },
              { reason: "'price' is required", metadata: null },
            ],
            [{}, { reason: "'price' is required" }],
          ];

          for (const [provided, expected] of responses) {
            const Book = new Schema({
              bookId: { required: true, validator },
              isPublished: { default: false, validator },
              name: { default: "", validator },
              price: {
                default: null,
                required: () => [true, provided],
                validator: validator,
              },
            }).getModel();

            it("should reject with proper required error message at creation", async () => {
              const { data, error } = await Book.create({ bookId: 1 });

              expect(data).toBeNull();

              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: { price: expect.objectContaining(expected) },
              });
            });

            it("should reject with proper required error message during updates", async () => {
              const book = {
                bookId: 1,
                isPublished: false,
                name: "",
                price: null,
              };
              const { data, error } = await Book.update(book, {
                name: "yooo",
              });

              expect(data).toBeNull();

              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: { price: expect.objectContaining(expected) },
              });
            });
          }
        });

        describe("should ignore unsupported types", () => {
          const invalidMessages = [
            null,
            undefined,
            [],
            {},
            1,
            0,
            -12,
            () => {},
          ];

          for (const message of invalidMessages) {
            const Book = new Schema({
              bookId: { required: true, validator },
              isPublished: { default: false, validator },
              name: { default: "", validator },
              price: {
                default: null,
                required: () => [true, message],
                validator: validator,
              },
            }).getModel();

            it("should reject with proper required error message at creation", async () => {
              const { data, error } = await Book.create({ bookId: 1 });

              expect(data).toBeNull();

              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  price: {
                    reason: "'price' is required",
                    metadata: null,
                  },
                },
              });
            });

            it("should reject with proper required error message during updates", async () => {
              const book = {
                bookId: 1,
                isPublished: false,
                name: "",
                price: null,
              };
              const { data, error } = await Book.update(book, {
                name: "yooo",
              });

              expect(data).toBeNull();

              expect(error).toMatchObject({
                message: ERRORS.VALIDATION_ERROR,
                payload: {
                  price: { reason: "'price' is required", metadata: null },
                },
              });
            });
          }
        });
      });

      describe("behaviour when a value returned by required function is not boolean nor array", () => {
        const invalidResponses = [
          null,
          undefined,
          {},
          "",
          "not array",
          1,
          0,
          -12,
          () => {},
        ];

        for (const response of invalidResponses) {
          const Book = new Schema({
            bookId: { required: true, validator },
            isPublished: { default: false, validator },
            name: { default: "", validator },
            price: {
              default: null,
              required: () => response,
              validator: validator,
            },
          }).getModel();

          it("should create normally", async () => {
            const { data } = await Book.create({ bookId: 1 });

            expect(data).toEqual({
              bookId: 1,
              isPublished: false,
              name: "",
              price: null,
            });
          });

          it("should update normally", async () => {
            const book = {
              bookId: 1,
              isPublished: false,
              name: "",
              price: null,
            };
            const { data } = await Book.update(book, { name: "yooo" });

            expect(data).toEqual({ name: "yooo" });
          });
        }
      });

      describe("behaviour with virtual properties", () => {
        const book = { name: "book name", price: 10 };

        describe("when value of virtual is not provided", () => {
          const Book = new Schema({
            name: { default: "" },
            price: {
              default: null,
              dependsOn: "_price",
              resolver: ({ context: { _price } }: any) => _price,
            },
            _price: {
              virtual: true,
              required({ context: { _price } }: any) {
                return _price == undefined;
              },
              validator: validator,
            },
          }).getModel();

          it("should reject at creation", async () => {
            const { data, error } = await Book.create({});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: {
                  reason: "'_price' is required",
                  metadata: null,
                },
              },
            });
          });

          it("should reject during updates", async () => {
            const { data, error } = await Book.update(book, {
              name: "updated name",
            });

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: {
                  reason: "'_price' is required",
                  metadata: null,
                },
              },
            });
          });
        });

        describe("when value of virtual is not provided and required at creation only", () => {
          const Book = new Schema({
            name: { default: "" },
            price: {
              default: null,
              dependsOn: "_price",
              resolver: ({ context: { _price } }: any) => _price,
            },
            _price: {
              virtual: true,
              required({ context: { _price }, isUpdate }: any) {
                return _price == undefined && !isUpdate;
              },
              validator: validator,
            },
          }).getModel();

          it("should reject at creation", async () => {
            const { data, error } = await Book.create({});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: {
                  reason: "'_price' is required",
                  metadata: null,
                },
              },
            });
          });

          it("should reject during updates", async () => {
            const name = "updated book name";
            const { data, error } = await Book.update(book, {
              name,
            });

            expect(error).toBeNull();
            expect(data).toEqual({ name });
          });
        });

        describe("when value of virtual is not provided and required at creation and update is blocked", () => {
          const Book = new Schema({
            name: { default: "" },
            price: {
              default: null,
              dependsOn: "_price",
              resolver: ({ context: { _price } }: any) => _price,
            },
            _price: {
              virtual: true,
              shouldUpdate: false,
              required({ context: { _price } }: any) {
                return _price == undefined;
              },
              validator: validator,
            },
          }).getModel();

          it("should reject at creation", async () => {
            const { data, error } = await Book.create({});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: {
                  reason: "'_price' is required",
                  metadata: null,
                },
              },
            });
          });

          it("should reject during updates", async () => {
            const name = "updated book name";
            const { data, error } = await Book.update(book, {
              name,
            });

            expect(error).toBeNull();
            expect(data).toEqual({ name });
          });
        });
      });

      describe("behaviour with asychronous required setters", () => {
        const book = { name: "book name", price: 10 };

        const Book = new Schema({
          name: { default: "" },
          price: {
            default: null,
            dependsOn: "_price",
            resolver: ({ context: { _price } }: any) => _price,
          },
          _price: {
            virtual: true,
            required({ context: { _price } }: any) {
              return Promise.resolve(_price == undefined);
            },
            validator: validator,
          },
        }).getModel();

        describe("creation", () => {
          it("should reject when condition is not met", async () => {
            const { data, error } = await Book.create({});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: { reason: "'_price' is required", metadata: null },
              },
            });
          });

          it("should allow when condition is met", async () => {
            const { data, error } = await Book.create({ _price: 20 });

            expect(error).toBeNull();
            expect(data).toMatchObject({ name: "", price: 20 });
          });
        });

        describe("updates", () => {
          it("should reject when condition is not met", async () => {
            const { data, error } = await Book.update(book, {});

            expect(data).toBeNull();
            expect(error).toMatchObject({
              message: ERRORS.VALIDATION_ERROR,
              payload: {
                _price: {
                  reason: "'_price' is required",
                  metadata: null,
                },
              },
            });
          });

          it("should allow when condition is met", async () => {
            const { data, error } = await Book.update(book, { _price: 20 });

            expect(error).toBeNull();
            expect(data).toMatchObject({ price: 20 });
          });
        });
      });

      describe("behaviour with errors thrown in required setter", () => {
        const Model = new Schema({
          prop1: { default: "" },
          prop: {
            default: null,
            required() {
              throw new Error("lolol");
            },
            validator,
          },
        }).getModel();

        it("should consider required:false if occurred at creation", async () => {
          const { data, error } = await Model.create();

          expect(error).toBeNull();
          expect(data).toEqual({ prop: null, prop1: "" });
        });

        it("should consider required:false if occurred during updates", async () => {
          const { data, error } = await Model.update(
            { prop: null, prop1: "" },
            { prop1: "updated" },
          );

          expect(error).toBeNull();
          expect(data).toEqual({ prop1: "updated" });
        });
      });
    });

    describe("valid", () => {
      it("should accept requiredBy + default(any | function)", () => {
        const values = ["", () => ""];

        for (const value of values) {
          const toPass = fx({
            propertyName: {
              default: value,
              required: () => true,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        }
      });

      it("should accept requiredBy + readonly", () => {
        const toPass = fx({
          propertyName: {
            default: "",
            readonly: true,
            required: () => true,
            validator,
          },
        });

        expectNoFailure(toPass);

        toPass();
      });

      it("should accept requiredBy + shouldInit", () => {
        const toPass = fx({
          propertyName: {
            default: "",
            readonly: true,
            required: () => true,
            shouldInit: () => true,
            validator,
          },
        });

        expectNoFailure(toPass);

        toPass();
      });
    });

    describe("invalid", () => {
      it("should reject requiredBy & no default", () => {
        const toFail = fx({
          propertyName: { required: () => true, validator },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Callable required properties must have a default value or setter",
              ]),
            }),
          );
        }
      });

      it("should reject requiredBy + default & dependent(true)", () => {
        const toFail = fx({
          dependentProp: {
            default: "value",
            dependsOn: "prop",
            resolver: () => 1,
            required: () => true,
            validator,
          },
          prop: { default: "" },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              dependentProp: expect.arrayContaining([
                "Required properties cannot be dependent",
              ]),
            }),
          );
        }
      });

      it("should reject requiredBy + allow", () => {
        const toFail = fx({
          prop: { required: () => true, allow: [1, 2, 435, 45], validator },
        });

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toMatchObject(
            expect.objectContaining({
              prop: expect.arrayContaining([
                '"allow" rule is cannot be applied to conditionally required properties',
              ]),
            }),
          );
        }
      });
    });
  });
};
