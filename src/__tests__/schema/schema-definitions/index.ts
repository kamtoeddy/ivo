export const schemaDefinition_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined, options: any = { timestamps: false }) =>
    () =>
      new Schema(definition, options);

  const expectFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).toThrow(message);
  };

  const expectPromiseFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).rejects.toThrow(message);
  };

  const expectNoFailure = (fx: Function) => {
    expect(fx).not.toThrow();
  };

  const validator = () => ({ valid: true });

  describe("Schema definitions", () => {
    it("should reject if property definitions is not an object", () => {
      const values = [
        null,
        undefined,
        new Number(),
        new String(),
        Symbol(),
        2,
        -10,
        true,
        [],
      ];

      for (const value of values) expectFailure(fx(value));
    });

    it("should reject if property definitions has no property", () => {
      const toFail = fx({});

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          "schema properties": ["Insufficient Schema properties"],
        });
      }
    });

    it("should reject if a property's definition is an empty object", () => {
      const toFail = fx({ emptyProp: {} });
      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err).toEqual(
          expect.objectContaining({
            message: "Invalid Schema",
            payload: {
              emptyProp: [
                "A property should at least be readonly, required, or have a default value",
              ],
            },
            statusCode: 500,
          })
        );
      }
    });

    describe("constant", () => {
      describe("valid", () => {
        let User: any, user: any;

        beforeAll(async () => {
          User = new Schema(
            {
              asyncConstant: { constant: true, value: asyncSetter },
              id: {
                constant: true,
                onCreate: ({ laxProp }: any) => ({ laxProp: laxProp + 2 }),
                value: (ctx: any) => (ctx?.id === "id" ? "id-2" : "id"),
              },
              parentId: { constant: true, value: "parent id" },
              laxProp: {
                default: 0,
                onUpdate: ({ laxProp }: any) => {
                  if (laxProp === "update id") return { id: "new id" };
                },
              },
            },
            { errors: "throw" }
          ).getModel();

          function asyncSetter() {
            return new Promise((res, rej) => {
              setTimeout(() => res(20), 500);
            });
          }

          user = (await User.create({ id: 2, parentId: [], laxProp: 2 })).data;
        });

        it("should set constants at creation", () => {
          expect(user).toEqual({
            asyncConstant: 20,
            id: "id",
            parentId: "parent id",
            laxProp: 4,
          });
        });

        it("should set constants during cloning", async () => {
          const { data: clone } = await User.clone(user, { reset: "laxProp" });

          expect(clone).toEqual({
            asyncConstant: 20,
            id: "id-2",
            parentId: "parent id",
            laxProp: 2,
          });
        });

        it("should not set constants via listeners", async () => {
          const { data: update } = await User.update(user, {
            laxProp: "update id",
          });

          expect(update).toEqual({ laxProp: "update id" });
        });

        it("should ignore constants during updates", () => {
          const toFail = User.update(user, { id: 25 });

          expect(toFail).rejects.toThrow("Nothing to update");
        });

        it("should accept constant(true) & value(any | ()=>any)", () => {
          const values = ["", "value", 1, null, false, true, {}, [], () => 1];

          for (const value of values) {
            const toPass = fx({ propertyName: { constant: true, value } });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should accept constant & value + onCreate(function | function[])", () => {
          const values = [() => ({}), [() => ({})], [() => ({}), () => ({})]];

          for (const onCreate of values) {
            const toPass = fx({
              propertyName: { constant: true, value: "", onCreate },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should accept constant & value + onDelete(function | function[])", () => {
          const values = [() => ({}), [() => ({})], [() => ({}), () => ({})]];

          for (const onDelete of values) {
            const toPass = fx({
              propertyName: { constant: true, value: "", onDelete },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should accept constant & value + onSuccess(function | function[])", () => {
          const values = [() => ({}), [() => ({})], [() => ({}), () => ({})]];

          for (const onSuccess of values) {
            const toPass = fx({
              propertyName: { constant: true, value: "", onSuccess },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });
      });

      describe("invalid", () => {
        it("should reject constant(!true)", () => {
          const values = [1, "", null, undefined, false, {}, []];

          for (const value of values) {
            const toFail = fx({
              propertyName: { constant: value, value: "" },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Constant properties must have constant as 'true'",
                  ]),
                })
              );
            }
          }
        });

        it("should reject constant & no value", () => {
          const toFail = fx({ propertyName: { constant: true } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Constant properties must have a value or setter",
                ]),
              })
            );
          }
        });

        it("should reject constant & value(undefined)", () => {
          const toFail = fx({
            propertyName: { constant: true, value: undefined },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Constant properties cannot have 'undefined' as value",
                ]),
              })
            );
          }
        });

        it("should reject (constant & value) + any other rule(!onCreate)", () => {
          const rules = [
            "default",
            "dependent",
            "onChange",
            "onFailure",
            "onUpdate",
            "readonly",
            "required",
            "sideEffect",
            "shouldInit",
            "validator",
          ];

          for (const rule of rules) {
            const toFail = fx({
              propertyName: { constant: true, value: "", [rule]: true },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Constant properties can only have ('constant' & 'value') or 'onCreate' | 'onDelete' | 'onSuccess'",
                  ]),
                })
              );
            }
          }
        });
      });
    });

    describe("dependent", () => {
      describe("valid", () => {
        it("should accept dependent & default(any | function)", () => {
          const values = ["", 1, false, true, null, {}, []];

          for (const value of values) {
            const toPass = fx({
              propertyName: { default: value, dependent: true },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should accept life cycle listeners", () => {
          const lifeCycles = ["onCreate", "onChange", "onUpdate"];
          const values = [() => {}, () => ({}), [() => {}, () => ({})]];

          for (const lifeCycle of lifeCycles) {
            for (const value of values) {
              const toPass = fx({
                propertyName: {
                  default: value,
                  dependent: true,
                  [lifeCycle]: value,
                },
              });

              expectNoFailure(toPass);

              toPass();
            }
          }
        });

        it("should accept validator", () => {
          const toPass = fx({
            propertyName: { default: "", dependent: true, validator },
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe("invalid", () => {
        it("should reject dependent & no default", () => {
          const toFail = fx({ propertyName: { dependent: true } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Dependent properties must have a default value",
                ]),
              })
            );
          }
        });

        it("should reject dependent & shouldInit", () => {
          const values = [false, true];

          for (const shouldInit of values) {
            const toFail = fx({
              propertyName: { default: "", dependent: true, shouldInit },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Dependent properties cannot have shouldInit rule",
                  ]),
                })
              );
            }
          }
        });

        it("should reject dependent & readonly(lax)", () => {
          const toFail = fx({
            propertyName: {
              default: "",
              dependent: true,
              readonly: "lax",
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Dependent properties cannot be readonly 'lax'",
                ]),
              })
            );
          }
        });

        it("should reject dependent & required", () => {
          const toFail = fx({
            propertyName: {
              default: "",
              dependent: true,
              required: true,
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Dependent properties cannot be strictly required",
                ]),
              })
            );
          }
        });
      });
    });

    describe("lax props", () => {
      describe("valid", () => {
        it("should allow default alone", () => {
          const toPass = fx({ propertyName: { default: "" } });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow default + validator", () => {
          const toPass = fx({
            propertyName: { default: "", validtor: () => ({ valid: true }) },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow default + (onChange | onChange[])", () => {
          const values = [() => {}, [() => {}], [() => {}, () => {}]];

          for (const onChange of values) {
            const toPass = fx({
              dependent: { default: "", dependent: true },
              propertyName: { default: "", onChange },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow default + (onCreate | onCreate[])", () => {
          const values = [() => {}, [() => {}], [() => {}, () => {}]];

          for (const onCreate of values) {
            const toPass = fx({
              dependent: { default: "", dependent: true },
              propertyName: { default: "", onCreate },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow default + (onUpdate | onUpdate[])", () => {
          const values = [() => {}, [() => {}], [() => {}, () => {}]];

          for (const onUpdate of values) {
            const toPass = fx({
              dependent: { default: "", dependent: true },
              propertyName: { default: "", onUpdate },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow default + onChange + onCreate + onUpdate + validator", () => {
          const toPass = fx({
            propertyName: {
              default: "",
              onChange: () => ({}),
              onCreate: [() => ({})],
              onUpdate: () => ({}),
              validtor: () => ({ valid: true }),
            },
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe("invalid", () => {
        it("should reject no default", () => {
          const toFail = fx({
            propertyName: { validator: () => ({ valid: true }) },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "A property should at least be readonly, required, or have a default value",
                ]),
              })
            );
          }
        });

        it("should reject default + invalid onChange", () => {
          const values = [false, 1, "", undefined, true, null];

          for (const onChange of values) {
            const toFail = fx({ propertyName: { default: "", onChange } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload.propertyName.length).toBe(1);
            }
          }
        });

        it("should reject default + invalid onCreate", () => {
          const values = [false, 1, "", undefined, true, null];

          for (const onCreate of values) {
            const toFail = fx({ propertyName: { default: "", onCreate } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload.propertyName.length).toBe(1);
            }
          }
        });

        it("should reject default + invalid onUpdate", () => {
          const values = [false, 1, "", undefined, true, null];

          for (const onUpdate of values) {
            const toFail = fx({ propertyName: { default: "", onUpdate } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload.propertyName.length).toBe(1);
            }
          }
        });
      });
    });

    describe("life cycle listeners", () => {
      const rules = [
        "onChange",
        "onCreate",
        "onDelete",
        "onFailure",
        "onSuccess",
        "onUpdate",
      ];

      describe("valid", () => {
        const values = [
          () => {},
          () => ({}),
          [() => {}],
          [() => {}, () => ({})],
        ];

        for (const rule of rules) {
          for (const value of values) {
            const toPass = fx({
              propertyName: {
                default: "",
                [rule]: value,
                validator,
              },
            });

            expectNoFailure(toPass);

            toPass();
          }
        }
      });

      describe("invalid", () => {
        const values = [1, "", 0, false, true, null, {}];

        for (const rule of rules) {
          for (const value of values) {
            const toFail = fx({
              propertyName: {
                default: "",
                [rule]: value,
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    `The listener for '${rule}' @[0] is not a function`,
                  ]),
                })
              );
            }
          }
        }
      });

      describe("onDelete", () => {
        let Model: any,
          propChangeMap: any = {};

        beforeAll(() => {
          const onDelete =
            (prop = "") =>
            () =>
              (propChangeMap[prop] = true);
          const validator = () => ({ valid: false });

          Model = new Schema({
            constant: {
              constant: true,
              value: "constant",
              onDelete: onDelete("constant"),
            },
            prop1: { required: true, onDelete: onDelete("prop1"), validator },
            prop2: { required: true, onDelete: onDelete("prop2"), validator },
            prop3: { required: true, onDelete: onDelete("prop3"), validator },
          }).getModel();
        });

        beforeEach(() => (propChangeMap = {}));

        it("should trigger all onDelete listeners but for sideEffects", async () => {
          await Model.delete({
            constant: true,
            prop1: true,
            prop2: true,
            prop3: true,
            prop4: true,
          });

          expect(propChangeMap).toEqual({
            constant: true,
            prop1: true,
            prop2: true,
            prop3: true,
          });
        });
      });

      describe("onFailure", () => {
        let Model: any,
          propChangeMap: any = {};

        beforeAll(() => {
          const onFailure =
            (prop = "") =>
            () =>
              (propChangeMap[prop] = true);
          const validator = () => ({ valid: false });

          Model = new Schema({
            prop1: { required: true, onFailure: onFailure("prop1"), validator },
            prop2: { required: true, onFailure: onFailure("prop2"), validator },
            prop3: { required: true, onFailure: onFailure("prop3"), validator },
            prop4: {
              sideEffect: true,
              onChange: validator,
              onFailure: onFailure("prop4"),
              validator,
            },
          }).getModel();
        });

        beforeEach(() => (propChangeMap = {}));

        // creation
        it("should call onFailure listeners at creation", async () => {
          const { error } = await Model.create({});

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({
            prop1: true,
            prop2: true,
            prop3: true,
          });
        });

        it("should call onFailure listeners at creation with sideEffects", async () => {
          const { error } = await Model.create({ prop4: "Yes" });

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({
            prop1: true,
            prop2: true,
            prop3: true,
            prop4: true,
          });
        });

        // cloning
        it("should call onFailure listeners during cloning", async () => {
          const { error } = await Model.clone({});

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({
            prop1: true,
            prop2: true,
            prop3: true,
          });
        });

        it("should call onFailure listeners during cloning with sideEffects", async () => {
          const { error } = await Model.clone({ prop4: "Yes" });

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({
            prop1: true,
            prop2: true,
            prop3: true,
            prop4: true,
          });
        });

        // updates
        it("should call onFailure listeners during updates", async () => {
          const { error } = await Model.update({}, { prop1: "" });

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({ prop1: true });
        });

        it("should call onFailure listeners during updates with sideEffects", async () => {
          const data = [
            [{ prop4: "" }, { prop4: true }],
            [
              { prop1: "", prop4: "" },
              { prop1: true, prop4: true },
            ],
          ];

          for (const [changes, results] of data) {
            const { error } = await Model.update({}, changes);

            expect(error).toBeDefined();
            expect(propChangeMap).toEqual(results);
          }
        });

        it("should call onFailure listeners during updates & nothing to update", async () => {
          const { error } = await Model.update({}, {});

          expect(error).toBeDefined();
          expect(propChangeMap).toEqual({});
        });
      });

      describe("onSuccess", () => {
        let Model: any,
          initialData = {
            dependent: "",
            lax: "changed",
            readonly: "changed",
            readonlyLax: "",
            required: "changed",
          },
          propChangeMap: any = {};

        beforeAll(() => {
          const onSuccess =
            (prop = "") =>
            () =>
              (propChangeMap[prop] = true);
          const validator = () => ({ valid: true });

          Model = new Schema({
            dependent: {
              default: "",
              dependent: true,
              onSuccess: onSuccess("dependent"),
            },
            lax: { default: "", onSuccess: onSuccess("lax"), validator },
            readonly: {
              readonly: true,
              onSuccess: onSuccess("readonly"),
              validator,
            },
            readonlyLax: {
              default: "",
              readonly: "lax",
              onSuccess: onSuccess("readonlyLax"),
              onChange: () => ({ dependent: true }),
              validator,
            },
            required: {
              required: true,
              onSuccess: onSuccess("required"),
              validator,
            },
          }).getModel();
        });

        beforeEach(() => (propChangeMap = {}));

        // creation
        it("should call onSuccess listeners at creation", async () => {
          const { error, handleSuccess } = await Model.create({
            required: true,
            readonly: true,
          });

          await handleSuccess();

          expect(error).toBeUndefined();
          expect(propChangeMap).toEqual({
            dependent: true,
            lax: true,
            readonly: true,
            readonlyLax: true,
            required: true,
          });
        });

        // cloning
        it("should call onSuccess listeners during cloning", async () => {
          const { error, handleSuccess } = await Model.clone({
            required: true,
            readonly: true,
          });

          await handleSuccess();

          expect(error).toBeUndefined();
          expect(propChangeMap).toEqual({
            dependent: true,
            lax: true,
            readonly: true,
            readonlyLax: true,
            required: true,
          });
        });

        // updates
        it("should call onSuccess listeners during updates with lax props", async () => {
          const { error, handleSuccess } = await Model.update(initialData, {
            lax: true,
          });

          await handleSuccess();

          expect(error).toBeUndefined();
          expect(propChangeMap).toEqual({
            lax: true,
          });
        });

        it("should call onSuccess listeners during updates with readonlyLax & dependent", async () => {
          const { error, handleSuccess } = await Model.update(initialData, {
            readonlyLax: true,
          });

          await handleSuccess();

          expect(error).toBeUndefined();
          expect(propChangeMap).toEqual({
            dependent: true,
            readonlyLax: true,
          });
        });
      });
    });

    describe("readonly", () => {
      describe("valid", () => {
        it("should allow readonly(true) + dependent + default", () => {
          const toPass = fx({
            propertyName: { readonly: true, dependent: true, default: "" },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow readonly(true) + requiredBy", () => {
          const toPass = fx({
            propertyName: {
              default: "",
              readonly: true,
              required: () => true,
              requiredError: "",
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe("invalid", () => {
        it("should reject !readonly(true | 'lax')", () => {
          const values = [1, "", null, undefined, false];

          for (const readonly of values) {
            const toFail = fx({ propertyName: { readonly } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Readonly properties are either true | 'lax'",
                  ]),
                })
              );
            }
          }
        });

        it("should reject readonly & required", () => {
          const values = [true, false];

          for (const required of values) {
            const toFail = fx({ propertyName: { readonly: true, required } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule",
                  ]),
                })
              );
            }
          }
        });

        it("should reject readonly(true) + dependent & no default", () => {
          const toFail = fx({
            propertyName: { readonly: true, dependent: true },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Dependent properties must have a default value",
                ]),
              })
            );
          }
        });

        it("should reject readonly(lax) + dependent", () => {
          const toFail = fx({
            propertyName: { default: "", readonly: "lax", dependent: true },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Readonly(lax) properties cannot be dependent",
                ]),
              })
            );
          }
        });

        it("should reject readonly(lax) & no default", () => {
          const toFail = fx({ propertyName: { readonly: "lax" } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "Readonly properties must have a default value or a default setter",
              ]),
            });
          }
        });

        it("should reject readonly(lax) & !shouldInit(undefined)", () => {
          const values = [false, true];

          for (const shouldInit of values) {
            const toFail = fx({
              propertyName: { default: "", readonly: "lax", shouldInit },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Lax properties cannot have initialization blocked",
                  ]),
                })
              );
            }
          }
        });
      });
    });

    describe("required", () => {
      describe("valid", () => {
        it("should allow required + validator", () => {
          const toPass = fx({ propertyName: { required: true, validator } });

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
              })
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
              })
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
                })
              );
            }
          }
        });

        it("should reject required(true) + dependent", () => {
          const values = [false, true, undefined];

          for (const dependent of values) {
            const toFail = fx({
              propertyName: { dependent, required: true, validator },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Required properties cannot be dependent",
                  ]),
                })
              );
            }
          }
        });

        it("should reject required(true) + requiredError", () => {
          const values = ["", () => ""];

          for (const requiredError of values) {
            const toFail = fx({
              propertyName: { required: true, requiredError, validator },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Strictly required properties cannot have a requiredError",
                  ]),
                })
              );
            }
          }
        });

        it("should reject required(true) + shouldInit", () => {
          const values = [false, true, () => "", [], {}];

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
                    "Required properties cannot have a initialization blocked",
                  ]),
                })
              );
            }
          }
        });
      });
    });

    describe("requiredBy", () => {
      describe("valid", () => {
        let Book: any, book: any;

        beforeAll(async () => {
          Book = new Schema(
            {
              bookId: {
                required: true,
                validator,
              },
              isPublished: {
                default: false,
                validator,
              },
              price: {
                default: null,
                required(ctx: any) {
                  return ctx.isPublished && ctx.price == null;
                },
                requiredError: "A price is required to publish a book!",
                validator: validatePrice,
              },
              priceReadonly: {
                default: null,
                readonly: true,
                required(ctx: any) {
                  return ctx.price == 101 && ctx.priceReadonly == null;
                },
                requiredError: "A priceReadonly is required when price is 101!",
                validator: validatePrice,
              },
            },
            { errors: "throw" }
          ).getModel();

          function validatePrice(price: any) {
            const validated = Number(price),
              valid = !isNaN(price) && validated;
            return { valid, validated };
          }

          book = (await Book.create({ bookId: 1 })).data;
        });

        it("should create normally", () => {
          expect(book).toEqual({
            bookId: 1,
            isPublished: false,
            price: null,
            priceReadonly: null,
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
          });
        });

        it("should reject if condition is not met at creation", async () => {
          const toFail = () => Book.create({ bookId: 1, isPublished: true });

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                price: expect.arrayContaining([
                  "A price is required to publish a book!",
                ]),
              })
            );
          }
        });

        it("should pass if condition is met during cloning", async () => {
          const toPass = () =>
            Book.clone({ bookId: 1, isPublished: true, price: 2000 });

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({
            bookId: 1,
            isPublished: true,
            price: 2000,
            priceReadonly: null,
          });
        });

        it("should reject if condition is not met during cloning", async () => {
          const toFail = () => Book.clone({ bookId: 1, isPublished: true });

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                price: expect.arrayContaining([
                  "A price is required to publish a book!",
                ]),
              })
            );
          }
        });

        it("should pass if condition is met during updates", async () => {
          const toPass = () =>
            Book.update(
              { bookId: 1, isPublished: false, price: null },
              { isPublished: true, price: 20 }
            );

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({ isPublished: true, price: 20 });
        });

        it("should pass if condition is met during updates of readonly", async () => {
          const toPass = () =>
            Book.update(book, { price: 101, priceReadonly: 201 });

          expectNoFailure(toPass);

          const { data } = await toPass();

          expect(data).toEqual({ price: 101, priceReadonly: 201 });
        });

        it("should reject if condition is not met during updates", async () => {
          const toFail = () =>
            Book.update(
              { bookId: 1, isPublished: false, price: null },
              { isPublished: true }
            );

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                price: expect.arrayContaining([
                  "A price is required to publish a book!",
                ]),
              })
            );
          }
        });

        it("should reject if condition is not met during updates of readonly", async () => {
          const toFail = () => Book.update(book, { price: 101 });

          expectPromiseFailure(toFail, "Validation Error");

          try {
            await toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                priceReadonly: expect.arrayContaining([
                  "A priceReadonly is required when price is 101!",
                ]),
              })
            );
          }
        });

        it("should not update callable readonly prop that has changed", async () => {
          const toFail = () =>
            Book.update(
              {
                bookId: 1,
                isPublished: false,
                price: null,
                priceReadonly: 201,
              },
              { priceReadonly: 101 }
            );

          expectPromiseFailure(toFail, "Nothing to update");
        });

        it("should accept requiredBy + default(any | function)", () => {
          const values = ["", () => ""];

          for (const value of values) {
            const toPass = fx({
              propertyName: {
                default: value,
                required: () => true,
                requiredError: "",
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
              requiredError: "",
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should accept requiredBy + requiredError(string | function)", () => {
          const values = ["", () => ""];

          for (const requiredError of values) {
            const toPass = fx({
              propertyName: {
                default: "",
                required: () => true,
                requiredError,
                validator,
              },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should reject required(true) + shouldInit", () => {
          const values = [false, true, () => "", [], {}];

          for (const shouldInit of values) {
            const toFail = fx({
              propertyName: {
                default: "",
                required: () => true,
                requiredError: "",
                shouldInit,
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "Required properties cannot have a initialization blocked",
                  ]),
                })
              );
            }
          }
        });
      });

      describe("invalid", () => {
        it("should reject requiredBy & no default", () => {
          const toFail = fx({
            propertyName: {
              required: () => true,
              requiredError: "",
              validator,
            },
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
              })
            );
          }
        });

        it("should reject requiredError & required(!function)", () => {
          const values = [undefined, true, [], {}, false, 2, ""];

          for (const required of values) {
            const toFail = fx({
              propertyName: {
                default: true,
                required,
                requiredError: () => "",
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "RequiredError can only be used with a callable required rule",
                  ]),
                })
              );
            }
          }
        });

        it("should reject requiredBy & no requiredError", () => {
          const toFail = fx({
            propertyName: {
              default: "",
              required: () => true,
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Callable required properties must have a requiredError or setter",
                ]),
              })
            );
          }
        });

        it("should reject requiredBy + requiredError(!string & !function)", () => {
          const values = [1, {}, [], false, true, undefined];

          for (const requiredError of values) {
            const toFail = fx({
              propertyName: {
                default: "",
                required: () => true,
                requiredError,
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toMatchObject(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "RequiredError must be a string or setter",
                  ]),
                })
              );
            }
          }
        });

        it("should reject requiredBy + default & dependent(true)", () => {
          const toFail = fx({
            propertyName: {
              default: "",
              dependent: true,
              required: () => true,
              requiredError: "",
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Required properties cannot be dependent",
                ]),
              })
            );
          }
        });
      });
    });

    describe("shouldInit", () => {
      it("should reject shouldInit(false) & no default", () => {
        try {
          fx({ propertyName: { shouldInit: false } })();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              propertyName: expect.arrayContaining([
                "A property with initialization blocked must have a default value",
              ]),
            })
          );
        }
      });
    });

    describe("sideEffect", () => {
      describe("valid", () => {
        let User: any;

        beforeAll(() => {
          User = new Schema(
            {
              dependentSideNoInit: {
                default: "",
                dependent: true,
              },
              dependentSideInit: {
                default: false,
                dependent: true,
                validator: validateBoolean,
              },
              sideNoInit: {
                sideEffect: true,
                shouldInit: false,
                onChange: () => ({ dependentSideNoInit: "changed" }),
                validator: validateBoolean,
              },
              sideInit: {
                sideEffect: true,
                onChange: onSideInitChange,
                validator: validateBoolean,
              },
            },
            { errors: "throw" }
          ).getModel();

          function onSideInitChange({ sideInit }: any) {
            return { dependentSideInit: sideInit ? true : false };
          }

          function validateBoolean(value: any) {
            if (![false, true].includes(value))
              return { valid: false, reason: `${value} is not a boolean` };
            return { valid: true };
          }
        });

        it("should respect sideInits & sideNoInit", async () => {
          const { data: user } = await User.create({
            sideInit: true,
            name: "Peter",
          });

          expect(user).toEqual({
            dependentSideNoInit: "",
            dependentSideInit: true,
          });
        });

        it("should allow onChange", () => {
          const toPass = fx({
            propertyName: { sideEffect: true, onChange: validator, validator },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow onFailure", () => {
          const toPass = fx({
            propertyName: {
              sideEffect: true,
              onChange: validator,
              onFailure: validator,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow requiredBy + requiredError", () => {
          const toPass = fx({
            propertyName: {
              sideEffect: true,
              onChange: validator,
              required: () => true,
              requiredError: () => "",
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow onChange + shouldInit(false) + validator", () => {
          const toPass = fx({
            propertyName: {
              sideEffect: true,
              shouldInit: false,
              onChange: validator,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        describe("RequiredSideEffect", () => {
          let RequiredSideEffect: any;

          beforeAll(() => {
            RequiredSideEffect = new Schema({
              dependent: { default: "", dependent: true },
              laxProp: { default: "" },
              sideEffect: {
                sideEffect: true,
                onChange,
                required: ({ sideEffect, dependent }: any) => {
                  return dependent === "" && sideEffect === undefined;
                },
                requiredError: () => "SideEffect is required",
                validator,
              },
            }).getModel();

            function onChange({ sideEffect }: any) {
              return { dependent: sideEffect };
            }
          });

          // creation
          it("should create normally", async () => {
            const { data } = await RequiredSideEffect.create({
              sideEffect: true,
              laxProp: "laxProp",
            });

            expect(data).toEqual({ dependent: true, laxProp: "laxProp" });
          });

          // cloning
          it("should clone normally", async () => {
            const { data } = await RequiredSideEffect.clone({
              sideEffect: "cloned",
              dependent: true,
              laxProp: "laxProp",
            });

            expect(data).toEqual({ dependent: "cloned", laxProp: "laxProp" });
          });

          it("should require during cloning", async () => {
            const { data, error } = await RequiredSideEffect.clone({
              dependent: "",
              laxProp: "laxProp",
            });

            expect(data).toBe(undefined);
            expect(error).toEqual(
              expect.objectContaining({
                message: "Validation Error",
                payload: { sideEffect: ["SideEffect is required"] },
                statusCode: 400,
              })
            );
          });

          // updates
          it("should update normally", async () => {
            const { data } = await RequiredSideEffect.update(
              {
                dependent: true,
                laxProp: "laxProp",
              },
              { sideEffect: "updated" }
            );

            expect(data).toEqual({ dependent: "updated" });
          });

          it("should require during updates", async () => {
            const { data, error } = await RequiredSideEffect.update(
              {
                dependent: "",
                laxProp: "laxProp",
              },
              { laxProp: 2 }
            );

            expect(data).toBe(undefined);
            expect(error).toEqual(
              expect.objectContaining({
                message: "Validation Error",
                payload: { sideEffect: ["SideEffect is required"] },
                statusCode: 400,
              })
            );
          });
        });
      });

      describe("invalid", () => {
        it("should reject sideEffect & no onChange listeners", () => {
          const toFail = fx({ propertyName: { sideEffect: true, validator } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "SideEffects must have at least one onChange listener",
                ]),
              })
            );
          }
        });

        it("should reject sideEffect & no validator ", () => {
          const toFail = fx({
            propertyName: { sideEffect: true, onChange: validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining(["Invalid validator"]),
              })
            );
          }
        });

        it("should reject shouldInit(!false)", () => {
          const values = [true, null, [], {}, "", 25];

          for (const shouldInit of values) {
            const toFail = fx({
              propertyName: {
                sideEffect: true,
                shouldInit,
                onChange: validator,
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "To block the initialization of side effects shouldInit must be 'false'",
                  ]),
                })
              );
            }
          }
        });

        it("should reject requiredBy + shouldInit", () => {
          const toFail = fx({
            propertyName: {
              sideEffect: true,
              shouldInit: false,
              onChange: validator,
              required: () => true,
              requiredError: () => "",
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Required sideEffects cannot have initialization blocked",
                ]),
              })
            );
          }
        });

        it("should reject required(true)", () => {
          const toFail = fx({
            propertyName: {
              sideEffect: true,
              onChange: validator,
              required: true,
              validator,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Callable required properties must have required as a function",
                ]),
              })
            );
          }
        });

        it("should reject any non sideEffect rule", () => {
          const values = [
            "constant",
            "default",
            "dependent",
            "onCreate",
            "onDelete",
            "onSuccess",
            "onUpdate",
            "readonly",
            "value",
          ];

          for (const rule of values) {
            const toFail = fx({
              propertyName: {
                sideEffect: true,
                [rule]: true,
                onChange: validator,
                validator,
              },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "SideEffects properties can only have ('sideEffect' + 'onChange' + 'validator') or 'shouldInit'",
                  ]),
                })
              );
            }
          }
        });
      });
    });
  });

  describe("Schema options", () => {
    const validSchema = {
      propertyName1: { default: "", validator },
      propertyName2: { default: "", validator },
    };

    it("should reject non-object values", () => {
      const values = [null, false, true, 1, "abc", []];

      for (const options of values) {
        const toFail = fx(validSchema, options);

        expectFailure(toFail);

        try {
          toFail();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              "schema options": expect.arrayContaining(["Must be an object"]),
            })
          );
        }
      }
    });

    it("should reject empty objects", () => {
      const toFail = fx(validSchema, {});

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            "schema options": expect.arrayContaining(["Cannot be empty"]),
          })
        );
      }
    });

    it("should reject invalid option name", () => {
      const toFail = fx(validSchema, { propertyName: true });

      expectFailure(toFail);

      try {
        toFail();
      } catch (err: any) {
        expect(err.payload).toEqual(
          expect.objectContaining({
            propertyName: expect.arrayContaining(["Invalid option"]),
          })
        );
      }
    });

    describe("errors", () => {
      it("should allow 'silent' | 'throw'", () => {
        const values = ["silent", "throw"];

        for (const errors of values) {
          const toPass = fx(validSchema, { errors });

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
            const toFail = fx(validSchema, { errors });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  errors: expect.arrayContaining([
                    "should be 'silent' or 'throws'",
                  ]),
                })
              );
            }
          }
        });
      });
    });

    describe("timestamps", () => {
      describe("valid", () => {
        it("should allow true | false", () => {
          const values = [false, true];

          for (const timestamps of values) {
            const toPass = fx(validSchema, { timestamps });

            expectNoFailure(toPass);

            toPass();
          }
        });

        const inputValue = {
          propertyName1: "value1",
          propertyName2: "value2",
        };

        describe("timestamps(true)", () => {
          let Model: any, entity: any;

          beforeAll(async () => {
            Model = new Schema(validSchema, { timestamps: true }).getModel();

            entity = (await Model.create(inputValue)).data;
          });

          it("should populate createdAt & updatedAt at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).toHaveProperty("createdAt");
            expect(entity).toHaveProperty("updatedAt");
          });

          it("should populate createdAt & updatedAt during cloning", async () => {
            const { data: clone } = await Model.clone(entity, {
              reset: "propertyName2",
            });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).toHaveProperty("createdAt");
            expect(clone).toHaveProperty("updatedAt");
          });

          it("should populate updatedAt during updates", async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty("createdAt");
            expect(updates).toHaveProperty("updatedAt");
          });
        });

        describe("timestamps(createdAt:c_At)", () => {
          let Model: any, entity: any;

          beforeAll(async () => {
            Model = new Schema(validSchema, {
              timestamps: { createdAt: "c_At" },
            }).getModel();

            entity = (await Model.create(inputValue)).data;
          });

          it("should populate c_At & updatedAt at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("createdAt");
            expect(entity).toHaveProperty("c_At");
            expect(entity).toHaveProperty("updatedAt");
          });

          it("should populate c_At & updatedAt during cloning", async () => {
            const { data: clone } = await Model.clone(entity, {
              reset: "propertyName2",
            });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).not.toHaveProperty("createdAt");
            expect(clone).toHaveProperty("c_At");
            expect(clone).toHaveProperty("updatedAt");
          });

          it("should populate updatedAt during updates", async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty("c_At");
            expect(updates).not.toHaveProperty("createdAt");
            expect(updates).toHaveProperty("updatedAt");
          });
        });

        describe("timestamps(updatedAt:u_At)", () => {
          let Model: any, entity: any;

          beforeAll(async () => {
            Model = new Schema(validSchema, {
              timestamps: { updatedAt: "u_At" },
            }).getModel();

            entity = (await Model.create(inputValue)).data;
          });

          it("should populate createdAt & u_At at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("updatedAt");
            expect(entity).toHaveProperty("createdAt");
            expect(entity).toHaveProperty("u_At");
          });

          it("should populate createdAt & u_At during cloning", async () => {
            const { data: clone } = await Model.clone(entity, {
              reset: "propertyName2",
            });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).not.toHaveProperty("updatedAt");
            expect(clone).toHaveProperty("createdAt");
            expect(clone).toHaveProperty("u_At");
          });

          it("should populate u_At during updates", async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty("createdAt");
            expect(updates).not.toHaveProperty("updatedAt");
            expect(updates).toHaveProperty("u_At");
          });
        });

        describe("timestamps(createdAt:c_At, updatedAt:u_At)", () => {
          let Model: any, entity: any;

          beforeAll(async () => {
            Model = new Schema(validSchema, {
              timestamps: { createdAt: "c_At", updatedAt: "u_At" },
            }).getModel();

            entity = (await Model.create(inputValue)).data;
          });

          it("should populate c_At & u_At at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("createdAt");
            expect(entity).not.toHaveProperty("updatedAt");
            expect(entity).toHaveProperty("c_At");
            expect(entity).toHaveProperty("u_At");
          });

          it("should populate c_At & u_At during cloning", async () => {
            const { data: clone } = await Model.clone(entity, {
              reset: "propertyName2",
            });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).not.toHaveProperty("createdAt");
            expect(clone).not.toHaveProperty("updatedAt");
            expect(clone).toHaveProperty("c_At");
            expect(clone).toHaveProperty("u_At");
          });

          it("should populate u_At during updates", async () => {
            const { data: updates } = await Model.update(entity, {
              propertyName2: 20,
            });

            expect(updates).toMatchObject({ propertyName2: 20 });

            expect(updates).not.toHaveProperty("createdAt");
            expect(updates).not.toHaveProperty("updatedAt");
            expect(updates).not.toHaveProperty("c_At");
            expect(updates).toHaveProperty("u_At");
          });
        });
      });

      describe("invalid", () => {
        it("should reject non boolean & non objects", () => {
          const values = [null, [], 1, "2asf"];

          for (const timestamps of values) {
            const toFail = fx(validSchema, { timestamps });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  timestamps: expect.arrayContaining([
                    "should be 'boolean' or 'non null object'",
                  ]),
                })
              );
            }
          }
        });

        it("should reject empty object", () => {
          const toFail = fx(validSchema, { timestamps: {} });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                timestamps: expect.arrayContaining([
                  "cannot be an empty object",
                ]),
              })
            );
          }
        });

        it("should reject custom name found on schema", () => {
          const values = ["propertyName1", "propertyName2"];
          const timestampKeys = ["createdAt", "updatedAt"];

          for (const ts_key of timestampKeys) {
            for (const value of values) {
              const toFail = fx(validSchema, {
                timestamps: { [ts_key]: value },
              });

              expectFailure(toFail);

              try {
                toFail();
              } catch (err: any) {
                expect(err.payload).toEqual(
                  expect.objectContaining({
                    timestamps: expect.arrayContaining([
                      `'${value}' already belongs to your schema`,
                    ]),
                  })
                );
              }
            }
          }
        });

        it("should reject custom name found on schema", () => {
          const toFail = fx(validSchema, {
            timestamps: { createdAt: "c_At", updatedAt: "c_At" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                timestamps: expect.arrayContaining([
                  "createdAt & updatedAt cannot be same",
                ]),
              })
            );
          }
        });
      });
    });
  });
};
