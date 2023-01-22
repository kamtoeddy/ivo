const pauseFor = (ms = 5) =>
  new Promise((res) => setTimeout(() => res(true), ms));

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

        it("should reject 'value' on non-constants", () => {
          const toFail = fx({ propertyName: { value: true } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "'value' rule can only be used with constant properties",
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
            "dependsOn",
            "onChange",
            "onFailure",
            "onUpdate",
            "readonly",
            "resolver",
            "required",
            "sanitizer",
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
      const resolver = () => 1;

      describe("valid", () => {
        describe("behaviour", () => {
          let Model: any, data: any;

          let resolversCalledOnCreateStats = {} as Record<
            string,
            number | undefined
          >;

          beforeAll(async () => {
            Model = new Schema({
              laxProp: { default: "" },
              laxProp_1: { default: "" },
              laxProp_2: {
                default: "",
                onCreate: [
                  attemptToModify("dependentProp"),
                  attemptToModify("dependentProp_1"),
                  attemptToModify("dependentProp_2"),
                ],
                onChange: [
                  attemptToModify("dependentProp"),
                  attemptToModify("dependentProp_1"),
                  attemptToModify("dependentProp_2"),
                ],
                onUpdate: [
                  attemptToModify("dependentProp"),
                  attemptToModify("dependentProp_1"),
                  attemptToModify("dependentProp_2"),
                ],
              },
              dependentProp: {
                default: 0,
                dependent: true,
                dependsOn: ["laxProp", "laxProp_1"],
                resolver: resolverOfDependentProp,
              },
              dependentProp_1: {
                default: 0,
                dependent: true,
                dependsOn: "dependentProp",
                resolver: resolverOfDependentProp_1,
              },
              dependentProp_2: {
                default: 0,
                dependent: true,
                dependsOn: "dependentProp",
                readonly: true,
                resolver: asyncResolver,
              },
            }).getModel();

            function attemptToModify(prop: string) {
              return (ctx: any) => ({ [prop]: ctx[prop] + 1000 });
            }

            function incrementResolveCountFor(prop: string) {
              const previousCount = resolversCalledOnCreateStats[prop] ?? 0;

              resolversCalledOnCreateStats[prop] = previousCount + 1;
            }

            function resolverOfDependentProp({ laxProp, laxProp_1 }: any) {
              incrementResolveCountFor("dependentProp");
              return laxProp.length + laxProp_1.length;
            }

            function resolverOfDependentProp_1({ dependentProp }: any) {
              incrementResolveCountFor("dependentProp_1");

              return dependentProp + 1;
            }

            async function asyncResolver({ dependentProp }: any) {
              incrementResolveCountFor("dependentProp_2");

              await pauseFor();

              return dependentProp + 2;
            }

            const res = await Model.create({
              laxProp_2: "value based pricing",
              dependentProp: 25,
              dependentProp_1: 34,
              dependentProp_2: 17,
            });

            data = res.data;

            // await res.handleSuccess?.()
          });

          beforeEach(() => {
            let resolversCalledOnCreateStats = {};
          });

          describe("creation", () => {
            it("should have all correct properties and values at creation", () => {
              expect(data).toEqual({
                laxProp: "",
                laxProp_1: "",
                laxProp_2: "value based pricing",
                dependentProp: 0,
                dependentProp_1: 0,
                dependentProp_2: 0,
              });

              expect(resolversCalledOnCreateStats).toEqual({});
            });
          });

          describe("cloning", () => {
            it("should have all correct properties and values at creation with 'clone' method", async () => {
              const { data: clone } = await Model.clone(data);

              expect(clone).toMatchObject({
                laxProp: "",
                laxProp_1: "",
                laxProp_2: "value based pricing",
                dependentProp: 0,
                dependentProp_1: 0,
                dependentProp_2: 0,
              });

              expect(resolversCalledOnCreateStats).toEqual({});
            });

            it("should respect 'reset' option at creation with 'clone' method", async () => {
              const { data: clone } = await Model.clone(
                {
                  ...data,
                  dependentProp: 20,
                  dependentProp_1: 1302,
                  dependentProp_2: 10,
                },
                {
                  reset: [
                    "dependentProp",
                    "dependentProp_1",
                    "dependentProp_2",
                  ],
                }
              );

              expect(clone).toMatchObject({
                laxProp: "",
                laxProp_1: "",
                laxProp_2: "value based pricing",
                dependentProp: 0,
                dependentProp_1: 0,
                dependentProp_2: 0,
              });

              expect(resolversCalledOnCreateStats).toEqual({});
            });
          });

          describe("updates", () => {
            it("should have all correct properties and values after updates", async () => {
              const { data: updates } = await Model.update(data, {
                laxProp_2: "hey",
                dependentProp: 74,
                dependentProp_1: 235,
                dependentProp_2: 72,
              });

              expect(updates).toMatchObject({ laxProp_2: "hey" });

              expect(resolversCalledOnCreateStats).toEqual({});
            });
          });

          // describe("deletion", () => {
          //   it("should have all correct properties and values at creation", () => {
          //     expect(data).toEqual({
          //       laxProp: "value",
          //       laxProp_1: "based pricing",
          //       dependentProp: 0,
          //       dependentProp_1: 0,
          //       dependentProp_2: 0,
          //     });
          //   });
          // });
        });

        it("should accept dependent & default(any | function)", () => {
          const values = ["", 1, false, true, null, {}, []];

          for (const value of values) {
            const toPass = fx({
              dependentProp: {
                default: value,
                dependent: true,
                dependsOn: "prop",
                resolver,
              },
              prop: { default: "" },
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
                dependentProp: {
                  default: value,
                  dependent: true,
                  dependsOn: "prop",
                  resolver,
                  [lifeCycle]: value,
                },
                prop: { default: "" },
              });

              expectNoFailure(toPass);

              toPass();
            }
          }
        });

        it("should accept dependsOn & resolver", () => {
          const values = [
            "prop",
            ["prop", "prop1"],
            ["prop", "prop1", "prop2", "prop3"],
          ];

          for (const dependsOn of values) {
            const toPass = fx({
              dependentProp: {
                default: "",
                dependent: true,
                dependsOn,
                resolver,
              },
              prop: { default: "" },
              prop1: { default: "" },
              prop2: { default: "" },
              prop3: { default: "" },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow a dependent prop to depend on another dependent prop (non-circular)", () => {
          const toPass = fx({
            dependentProp1: {
              default: "",
              dependent: true,
              dependsOn: "prop",
              resolver,
            },
            dependentProp2: {
              default: "",
              dependent: true,
              dependsOn: "dependentProp1",
              resolver,
            },
            prop: { default: "" },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow a dependency on side effects", () => {
          const toPass = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "sideEFFectProp",
              resolver,
            },
            sideEFFectProp: { sideEffect: true, validator: resolver },
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe("invalid", () => {
        it("should reject dependency on non-properties", () => {
          const invalidProp = "invalidProp";

          const toFail = fx({
            dependentProp: {
              dependent: true,
              default: "",
              dependsOn: invalidProp,
              resolver,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  `Cannot establish dependency with '${invalidProp}' as it is neither a property nor a side effect of your model`,
                ]),
              })
            );
          }
        });

        it("should not allow property to depend on itself", () => {
          const toFail = fx({
            dependentProp: {
              dependent: true,
              default: "",
              dependsOn: "dependentProp",
              resolver,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "A property cannot depend on itself",
                ]),
              })
            );
          }
        });

        it("should not allow property to depend on a constant property", () => {
          const toFail = fx({
            constantProp: {
              constant: true,
              value: "",
            },
            dependentProp: {
              dependent: true,
              default: "",
              dependsOn: "constantProp",
              resolver,
            },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "A property cannot depend on a constant property",
                ]),
              })
            );
          }
        });

        it("should identify circular dependencies and reject", () => {
          const toFail = fx({
            A: {
              default: "",
              dependent: true,
              dependsOn: ["B", "C", "D"],
              resolver,
            },
            B: {
              default: "",
              dependent: true,
              dependsOn: ["A", "C", "E"],
              resolver,
            },
            C: {
              default: "",
              dependent: true,
              dependsOn: ["A"],
              resolver,
            },
            D: {
              default: "",
              dependent: true,
              dependsOn: "E",
              resolver,
            },
            E: {
              default: "",
              dependent: true,
              dependsOn: "A",
              resolver,
            },
            F: {
              default: "",
              dependent: true,
              dependsOn: "prop",
              resolver,
            },
            prop: { default: "" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                A: expect.arrayContaining([
                  "Circular dependency identified with 'B'",
                  "Circular dependency identified with 'C'",
                  "Circular dependency identified with 'E'",
                ]),
                B: expect.arrayContaining([
                  "Circular dependency identified with 'A'",
                ]),
                C: expect.arrayContaining([
                  "Circular dependency identified with 'A'",
                  "Circular dependency identified with 'B'",
                ]),
                D: expect.arrayContaining([
                  "Circular dependency identified with 'A'",
                ]),
                E: expect.arrayContaining([
                  "Circular dependency identified with 'B'",
                  "Circular dependency identified with 'D'",
                ]),
              })
            );
          }
        });

        it("should reject dependent + missing default", () => {
          const toFail = fx({ dependentProp: { dependent: true } });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "Dependent properties must have a default value",
                ]),
              })
            );
          }
        });

        it("should reject dependent + missing dependsOn", () => {
          const toFail = fx({
            propertyName: { dependent: true, default: "", resolver },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                propertyName: expect.arrayContaining([
                  "Dependent properties must depend on atleast one property",
                ]),
              })
            );
          }
        });

        it("should reject dependent + missing resolver", () => {
          const toFail = fx({
            dependentProp: { dependent: true, default: "", dependsOn: "prop" },
            prop: { default: "" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "Dependent properties must have a resolver",
                ]),
              })
            );
          }
        });

        it("should reject dependent & shouldInit", () => {
          const values = [false, true];

          for (const shouldInit of values) {
            const toFail = fx({
              dependentProp: {
                dependent: true,
                default: "",
                dependsOn: "prop",
                resolver,
                shouldInit,
              },
              prop: { default: "" },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  dependentProp: expect.arrayContaining([
                    "Dependent properties cannot have shouldInit rule",
                  ]),
                })
              );
            }
          }
        });

        it("should reject dependent & readonly(lax)", () => {
          const toFail = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "prop",
              resolver,
              readonly: "lax",
            },
            prop: { default: "" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "Dependent properties cannot be readonly 'lax'",
                ]),
              })
            );
          }
        });

        it("should reject dependent & validator", () => {
          const values = [null, "", 1, true, false, validator];

          for (const validator of values) {
            const toFail = fx({
              dependentProp: {
                default: "",
                dependent: true,
                dependsOn: "prop",
                resolver,
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
                    "Dependent properties cannot be validated",
                  ]),
                })
              );
            }
          }
        });

        it("should reject dependent & required", () => {
          const toFail = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "prop",
              resolver,
              required: true,
            },
            prop: { default: "" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "Dependent properties cannot be strictly required",
                ]),
              })
            );
          }
        });

        it("should reject no dependent + dependsOn or resolver", () => {
          const toFail = fx({
            dependentProp: { default: "", dependsOn: "prop", resolver },
            prop: { default: "" },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toMatchObject(
              expect.objectContaining({
                dependentProp: expect.arrayContaining([
                  "dependsOn & resolver rules can only belong to dependent properties",
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
            const toPass = fx({ prop: { default: "", onChange } });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow default + (onCreate | onCreate[])", () => {
          const values = [() => {}, [() => {}], [() => {}, () => {}]];

          for (const onCreate of values) {
            const toPass = fx({ prop: { default: "", onCreate } });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should allow default + (onUpdate | onUpdate[])", () => {
          const values = [() => {}, [() => {}], [() => {}, () => {}]];

          for (const onUpdate of values) {
            const toPass = fx({ prop: { default: "", onUpdate } });

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

    // describe("life cycle listeners", () => {
    //   const rules = [
    //     "onChange",
    //     "onCreate",
    //     "onDelete",
    //     "onFailure",
    //     "onSuccess",
    //     "onUpdate",
    //   ];

    //   describe("valid", () => {
    //     const values = [
    //       () => {},
    //       () => ({}),
    //       [() => {}],
    //       [() => {}, () => ({})],
    //     ];

    //     for (const rule of rules) {
    //       for (const value of values) {
    //         const toPass = fx({
    //           propertyName: {
    //             default: "",
    //             [rule]: value,
    //             validator,
    //           },
    //         });

    //         expectNoFailure(toPass);

    //         toPass();
    //       }
    //     }
    //   });

    //   describe("invalid", () => {
    //     const values = [1, "", 0, false, true, null, {}];

    //     for (const rule of rules) {
    //       for (const value of values) {
    //         const toFail = fx({
    //           propertyName: {
    //             default: "",
    //             [rule]: value,
    //             validator,
    //           },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 `The listener for '${rule}' @[0] is not a function`,
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     }
    //   });

    //   describe("life cycle readonly ctx", () => {
    //     const rules = [
    //       "onChange",
    //       "onCreate",
    //       "onDelete",
    //       "onFailure",
    //       "onSuccess",
    //       "onUpdate",
    //     ];

    //     let Model: any,
    //       propChangeMap: any = {};

    //     const validData = { constant: 1, prop1: "1", prop2: "2", prop3: "3" };
    //     const allProps = ["constant", "prop1", "prop2", "prop3"],
    //       props = ["prop1", "prop2", "prop3"];

    //     beforeAll(() => {
    //       const handle =
    //         (rule = "", prop = "") =>
    //         (ctx: any) => {
    //           try {
    //             ctx[prop] = 1;
    //           } catch (err) {
    //             if (!propChangeMap[rule]) propChangeMap[rule] = {};

    //             propChangeMap[rule][prop] = true;
    //           }
    //         };
    //       const validator = (value: any) => ({ valid: !!value });

    //       Model = new Schema({
    //         constant: {
    //           constant: true,
    //           value: "constant",
    //           onCreate: handle("onCreate", "constant"),
    //           onDelete: handle("onDelete", "constant"),
    //           onSuccess: handle("onSuccess", "constant"),
    //         },
    //         prop1: {
    //           required: true,
    //           onChange: handle("onChange", "prop1"),
    //           onCreate: handle("onCreate", "prop1"),
    //           onDelete: handle("onDelete", "prop1"),
    //           onFailure: handle("onFailure", "prop1"),
    //           onSuccess: handle("onSuccess", "prop1"),
    //           onUpdate: handle("onUpdate", "prop1"),
    //           validator,
    //         },
    //         prop2: {
    //           required: true,
    //           onChange: handle("onChange", "prop2"),
    //           onCreate: handle("onCreate", "prop2"),
    //           onDelete: handle("onDelete", "prop2"),
    //           onFailure: handle("onFailure", "prop2"),
    //           onSuccess: handle("onSuccess", "prop2"),
    //           onUpdate: handle("onUpdate", "prop2"),
    //           validator,
    //         },
    //         prop3: {
    //           required: true,
    //           onChange: handle("onChange", "prop3"),
    //           onCreate: handle("onCreate", "prop3"),
    //           onDelete: handle("onDelete", "prop3"),
    //           onFailure: handle("onFailure", "prop3"),
    //           onSuccess: handle("onSuccess", "prop3"),
    //           onUpdate: handle("onUpdate", "prop3"),
    //           validator,
    //         },
    //       }).getModel();
    //     });

    //     beforeEach(() => (propChangeMap = {}));

    //     it("should reject listeners that try to mutate the onChange, onCreate & onSuccess ctx", async () => {
    //       const { handleSuccess } = await Model.create(validData);

    //       await handleSuccess?.();

    //       for (const prop of allProps)
    //         expect(propChangeMap.onCreate[prop]).toBe(true);

    //       expect(propChangeMap.onSuccess.constant).toBe(true);

    //       for (const prop of props)
    //         for (const rule of ["onChange", "onSuccess"])
    //           expect(propChangeMap[rule][prop]).toBe(true);
    //     });

    //     it("should reject listeners that try to mutate the onChange, onCreate & onSuccess ctx (clone)", async () => {
    //       await Model.clone(validData);

    //       for (const prop of allProps)
    //         expect(propChangeMap.onCreate[prop]).toBe(true);

    //       for (const prop of props)
    //         expect(propChangeMap.onChange[prop]).toBe(true);
    //     });

    //     it("should reject listeners that try to mutate the onDelete ctx", async () => {
    //       await Model.delete(validData);

    //       for (const prop of allProps)
    //         expect(propChangeMap.onDelete[prop]).toBe(true);
    //     });

    //     it("should reject listeners that try to mutate the onFailure(clone) ctx", async () => {
    //       await Model.clone({ prop1: "", prop2: "", prop3: "" });

    //       for (const prop of props)
    //         for (const rule of rules) {
    //           const result = rule == "onFailure" ? true : undefined;

    //           expect(propChangeMap?.[rule]?.[prop]).toBe(result);
    //         }
    //     });

    //     it("should reject listeners that try to mutate the onFailure(create) ctx", async () => {
    //       await Model.create({ prop1: "", prop2: "", prop3: "" });

    //       for (const prop of props)
    //         for (const rule of rules) {
    //           const result = rule == "onFailure" ? true : undefined;

    //           expect(propChangeMap?.[rule]?.[prop]).toBe(result);
    //         }
    //     });

    //     it("should reject listeners that try to mutate the onFailure(update) ctx", async () => {
    //       await Model.update(validData, { prop1: "", prop2: "", prop3: "" });

    //       for (const prop of props)
    //         for (const rule of rules) {
    //           const result = rule == "onFailure" ? true : undefined;

    //           expect(propChangeMap?.[rule]?.[prop]).toBe(result);
    //         }
    //     });

    //     it("should reject listeners that try to mutate the onUpdate & onChange ctx", async () => {
    //       await Model.update(validData, { prop1: "2", prop2: "3", prop3: "4" });

    //       for (const prop of props) {
    //         expect(propChangeMap.onUpdate[prop]).toBe(true);
    //         expect(propChangeMap.onChange[prop]).toBe(true);
    //       }
    //     });
    //   });

    //   describe("onDelete", () => {
    //     let Model: any,
    //       propChangeMap: any = {};

    //     beforeAll(() => {
    //       const onDelete =
    //         (prop = "") =>
    //         () =>
    //           (propChangeMap[prop] = true);
    //       const validator = () => ({ valid: false });

    //       Model = new Schema({
    //         constant: {
    //           constant: true,
    //           value: "constant",
    //           onDelete: onDelete("constant"),
    //         },
    //         prop1: { required: true, onDelete: onDelete("prop1"), validator },
    //         prop2: { required: true, onDelete: onDelete("prop2"), validator },
    //         prop3: { required: true, onDelete: onDelete("prop3"), validator },
    //       }).getModel();
    //     });

    //     beforeEach(() => (propChangeMap = {}));

    //     it("should trigger all onDelete listeners but for sideEffects", async () => {
    //       await Model.delete({
    //         constant: true,
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //         prop4: true,
    //       });

    //       expect(propChangeMap).toEqual({
    //         constant: true,
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //       });
    //     });
    //   });

    //   describe("onFailure", () => {
    //     let Model: any,
    //       propChangeMap: any = {};

    //     beforeAll(() => {
    //       const onFailure =
    //         (prop = "") =>
    //         () =>
    //           (propChangeMap[prop] = true);
    //       const validator = () => ({ valid: false });

    //       Model = new Schema({
    //         prop1: { required: true, onFailure: onFailure("prop1"), validator },
    //         prop2: { required: true, onFailure: onFailure("prop2"), validator },
    //         prop3: { required: true, onFailure: onFailure("prop3"), validator },
    //         prop4: {
    //           sideEffect: true,
    //           onChange: validator,
    //           onFailure: onFailure("prop4"),
    //           validator,
    //         },
    //       }).getModel();
    //     });

    //     beforeEach(() => (propChangeMap = {}));

    //     // creation
    //     it("should call onFailure listeners at creation", async () => {
    //       const { error } = await Model.create({});

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //       });
    //     });

    //     it("should call onFailure listeners at creation with sideEffects", async () => {
    //       const { error } = await Model.create({ prop4: "Yes" });

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //         prop4: true,
    //       });
    //     });

    //     // cloning
    //     it("should call onFailure listeners during cloning", async () => {
    //       const { error } = await Model.clone({});

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //       });
    //     });

    //     it("should call onFailure listeners during cloning with sideEffects", async () => {
    //       const { error } = await Model.clone({ prop4: "Yes" });

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({
    //         prop1: true,
    //         prop2: true,
    //         prop3: true,
    //         prop4: true,
    //       });
    //     });

    //     // updates
    //     it("should call onFailure listeners during updates", async () => {
    //       const { error } = await Model.update({}, { prop1: "" });

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({ prop1: true });
    //     });

    //     it("should call onFailure listeners during updates with sideEffects", async () => {
    //       const data = [
    //         [{ prop4: "" }, { prop4: true }],
    //         [
    //           { prop1: "", prop4: "" },
    //           { prop1: true, prop4: true },
    //         ],
    //       ];

    //       for (const [changes, results] of data) {
    //         const { error } = await Model.update({}, changes);

    //         expect(error).toBeDefined();
    //         expect(propChangeMap).toEqual(results);
    //       }
    //     });

    //     it("should call onFailure listeners during updates & nothing to update", async () => {
    //       const { error } = await Model.update({}, {});

    //       expect(error).toBeDefined();
    //       expect(propChangeMap).toEqual({});
    //     });
    //   });

    //   describe("onSuccess", () => {
    //     let Model: any,
    //       initialData = {
    //         dependent: "",
    //         lax: "changed",
    //         readonly: "changed",
    //         readonlyLax: "",
    //         required: "changed",
    //       },
    //       propChangeMap: any = {};

    //     beforeAll(() => {
    //       const onSuccess =
    //         (prop = "") =>
    //         () =>
    //           (propChangeMap[prop] = true);
    //       const validator = () => ({ valid: true });

    //       Model = new Schema({
    //         dependent: {
    //           default: "",
    //           dependent: true,
    //           dependsOn: "readonly",
    //           onSuccess: onSuccess("dependent"),
    //         },
    //         lax: { default: "", onSuccess: onSuccess("lax"), validator },
    //         readonly: {
    //           readonly: true,
    //           onSuccess: onSuccess("readonly"),
    //           validator,
    //         },
    //         readonlyLax: {
    //           default: "",
    //           readonly: "lax",
    //           onSuccess: onSuccess("readonlyLax"),
    //           onChange: () => ({ dependent: true }),
    //           validator,
    //         },
    //         required: {
    //           required: true,
    //           onSuccess: onSuccess("required"),
    //           validator,
    //         },
    //       }).getModel();
    //     });

    //     beforeEach(() => (propChangeMap = {}));

    //     // creation
    //     it("should call onSuccess listeners at creation", async () => {
    //       const { error, handleSuccess } = await Model.create({
    //         required: true,
    //         readonly: true,
    //       });

    //       await handleSuccess();

    //       expect(error).toBeUndefined();
    //       expect(propChangeMap).toEqual({
    //         dependent: true,
    //         lax: true,
    //         readonly: true,
    //         readonlyLax: true,
    //         required: true,
    //       });
    //     });

    //     // cloning
    //     it("should call onSuccess listeners during cloning", async () => {
    //       const { error, handleSuccess } = await Model.clone({
    //         required: true,
    //         readonly: true,
    //       });

    //       await handleSuccess();

    //       expect(error).toBeUndefined();
    //       expect(propChangeMap).toEqual({
    //         dependent: true,
    //         lax: true,
    //         readonly: true,
    //         readonlyLax: true,
    //         required: true,
    //       });
    //     });

    //     // updates
    //     it("should call onSuccess listeners during updates with lax props", async () => {
    //       const { error, handleSuccess } = await Model.update(initialData, {
    //         lax: true,
    //       });

    //       await handleSuccess();

    //       expect(error).toBeUndefined();
    //       expect(propChangeMap).toEqual({
    //         lax: true,
    //       });
    //     });

    //     it("should call onSuccess listeners during updates with readonlyLax & dependent", async () => {
    //       const { error, handleSuccess } = await Model.update(initialData, {
    //         readonlyLax: true,
    //       });

    //       await handleSuccess();

    //       expect(error).toBeUndefined();
    //       expect(propChangeMap).toEqual({
    //         dependent: true,
    //         readonlyLax: true,
    //       });
    //     });
    //   });
    // });

    // describe("readonly", () => {
    //   describe("valid", () => {
    //     it("should allow readonly(true) + dependent + default", () => {
    //       const toPass = fx({
    //         dependentProp: {
    //           default: "value",
    //           dependent: true,
    //           dependsOn: "prop",
    //           resolver: () => 1,
    //           readonly: true,
    //         },
    //         prop: { default: "" },
    //       });

    //       expectNoFailure(toPass);

    //       toPass();
    //     });

    //     it("should allow readonly(true) + requiredBy", () => {
    //       const toPass = fx({
    //         propertyName: {
    //           default: "",
    //           readonly: true,
    //           required: () => true,
    //           requiredError: "",
    //           validator,
    //         },
    //       });

    //       expectNoFailure(toPass);

    //       toPass();
    //     });

    //     describe("behaviour", () => {
    //       let Model: any;

    //       beforeAll(() => {
    //         Model = new Schema({
    //           age: { readonly: true, default: null },
    //           name: {
    //             default: "Default Name",
    //             onChange: () => ({ age: 12 }),
    //             // onUpdate: () => ({ age: 12 }),
    //           },
    //         }).getModel();
    //       });

    //       it("should not modify readonly props that have changed via life cycle listeners at creation", async () => {
    //         const { data } = await Model.create({ age: 25 });

    //         expect(data).toMatchObject({ age: 25, name: "Default Name" });
    //       });

    //       it("should not modify readonly props that have changed via life cycle listeners during updates", async () => {
    //         const { data } = await Model.update(
    //           { age: null, name: "Default Name" },
    //           { age: 25, name: "YoYo" }
    //         );

    //         expect(data).toMatchObject({ age: 25, name: "YoYo" });
    //       });
    //     });
    //   });

    //   describe("invalid", () => {
    //     it("should reject !readonly(true | 'lax')", () => {
    //       const values = [1, "", null, undefined, false];

    //       for (const readonly of values) {
    //         const toFail = fx({ propertyName: { readonly } });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Readonly properties are either true | 'lax'",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject readonly & required", () => {
    //       const values = [true, false];

    //       for (const required of values) {
    //         const toFail = fx({ propertyName: { readonly: true, required } });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Strictly readonly properties are required. Either use a callable required + readonly(true) or remove the required rule",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject readonly(true) + dependent & no default", () => {
    //       const toFail = fx({
    //         dependentProp: {
    //           dependent: true,
    //           default: "",
    //           dependsOn: "prop",
    //           resolver: () => 1,
    //         },
    //         prop: { default: "" },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             dependentProp: expect.arrayContaining([
    //               "Dependent properties must have a default value",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject readonly(lax) + dependent", () => {
    //       const toFail = fx({
    //         dependentProp: {
    //           dependent: true,
    //           default: "",
    //           dependsOn: "prop",
    //           resolver: () => 1,
    //           readonly: "lax",
    //         },
    //         prop: { default: "" },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             dependentProp: expect.arrayContaining([
    //               "Readonly(lax) properties cannot be dependent",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject readonly(lax) & no default", () => {
    //       const toFail = fx({ propertyName: { readonly: "lax" } });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect.objectContaining({
    //           propertyName: expect.arrayContaining([
    //             "Readonly properties must have a default value or a default setter",
    //           ]),
    //         });
    //       }
    //     });

    //     it("should reject readonly(lax) & !shouldInit(undefined)", () => {
    //       const values = [false, true];

    //       for (const shouldInit of values) {
    //         const toFail = fx({
    //           propertyName: { default: "", readonly: "lax", shouldInit },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Lax properties cannot have initialization blocked",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });
    //   });
    // });

    // describe("required", () => {
    //   describe("valid", () => {
    //     it("should allow required + validator", () => {
    //       const toPass = fx({ propertyName: { required: true, validator } });

    //       expectNoFailure(toPass);

    //       toPass();
    //     });
    //   });

    //   describe("invalid", () => {
    //     it("should reject required & no validator", () => {
    //       const toFail = fx({ propertyName: { required: true } });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             propertyName: expect.arrayContaining([
    //               "Required properties must have a validator",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject required(true) + default", () => {
    //       const toFail = fx({
    //         propertyName: { default: "", required: true, validator },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             propertyName: expect.arrayContaining([
    //               "Strictly required properties cannot have a default value or setter",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject required(true) + readonly(true)", () => {
    //       const values = [false, true];

    //       for (const readonly of values) {
    //         const toFail = fx({
    //           propertyName: { readonly, required: true, validator },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Strictly required properties cannot be readonly",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject required(true) + dependent", () => {
    //       const values = [false, true, undefined];

    //       for (const dependent of values) {
    //         const toFail = fx({
    //           dependentProp: {
    //             dependent,
    //             default: "",
    //             dependsOn: "prop",
    //             resolver: () => 1,
    //           },
    //           prop: { default: "" },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               dependentProp: expect.arrayContaining([
    //                 "Required properties cannot be dependent",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject required(true) + requiredError", () => {
    //       const values = ["", () => ""];

    //       for (const requiredError of values) {
    //         const toFail = fx({
    //           propertyName: { required: true, requiredError, validator },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Strictly required properties cannot have a requiredError",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject required(true) + shouldInit", () => {
    //       const values = [false, true, () => "", [], {}];

    //       for (const shouldInit of values) {
    //         const toFail = fx({
    //           propertyName: { required: true, shouldInit, validator },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Required properties cannot have a initialization blocked",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });
    //   });
    // });

    // describe("requiredBy", () => {
    //   describe("valid", () => {
    //     let Book: any, book: any;

    //     beforeAll(async () => {
    //       Book = new Schema(
    //         {
    //           bookId: {
    //             required: true,
    //             validator,
    //           },
    //           isPublished: {
    //             default: false,
    //             validator,
    //           },
    //           price: {
    //             default: null,
    //             required(ctx: any) {
    //               return ctx.isPublished && ctx.price == null;
    //             },
    //             requiredError: "A price is required to publish a book!",
    //             validator: validatePrice,
    //           },
    //           priceReadonly: {
    //             default: null,
    //             readonly: true,
    //             required(ctx: any) {
    //               return ctx.price == 101 && ctx.priceReadonly == null;
    //             },
    //             requiredError: "A priceReadonly is required when price is 101!",
    //             validator: validatePrice,
    //           },
    //         },
    //         { errors: "throw" }
    //       ).getModel();

    //       function validatePrice(price: any) {
    //         const validated = Number(price),
    //           valid = !isNaN(price) && validated;
    //         return { valid, validated };
    //       }

    //       book = (await Book.create({ bookId: 1 })).data;
    //     });

    //     it("should create normally", () => {
    //       expect(book).toEqual({
    //         bookId: 1,
    //         isPublished: false,
    //         price: null,
    //         priceReadonly: null,
    //       });
    //     });

    //     it("should pass if condition is met at creation", async () => {
    //       const toPass = () =>
    //         Book.create({ bookId: 1, isPublished: true, price: 2000 });

    //       expectNoFailure(toPass);

    //       const { data } = await toPass();

    //       expect(data).toEqual({
    //         bookId: 1,
    //         isPublished: true,
    //         price: 2000,
    //         priceReadonly: null,
    //       });
    //     });

    //     it("should reject if condition is not met at creation", async () => {
    //       const toFail = () => Book.create({ bookId: 1, isPublished: true });

    //       expectPromiseFailure(toFail, "Validation Error");

    //       try {
    //         await toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             price: expect.arrayContaining([
    //               "A price is required to publish a book!",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should pass if condition is met during cloning", async () => {
    //       const toPass = () =>
    //         Book.clone({ bookId: 1, isPublished: true, price: 2000 });

    //       expectNoFailure(toPass);

    //       const { data } = await toPass();

    //       expect(data).toEqual({
    //         bookId: 1,
    //         isPublished: true,
    //         price: 2000,
    //         priceReadonly: null,
    //       });
    //     });

    //     it("should reject if condition is not met during cloning", async () => {
    //       const toFail = () => Book.clone({ bookId: 1, isPublished: true });

    //       expectPromiseFailure(toFail, "Validation Error");

    //       try {
    //         await toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             price: expect.arrayContaining([
    //               "A price is required to publish a book!",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should pass if condition is met during updates", async () => {
    //       const toPass = () =>
    //         Book.update(
    //           { bookId: 1, isPublished: false, price: null },
    //           { isPublished: true, price: 20 }
    //         );

    //       expectNoFailure(toPass);

    //       const { data } = await toPass();

    //       expect(data).toEqual({ isPublished: true, price: 20 });
    //     });

    //     it("should pass if condition is met during updates of readonly", async () => {
    //       const toPass = () =>
    //         Book.update(book, { price: 101, priceReadonly: 201 });

    //       expectNoFailure(toPass);

    //       const { data } = await toPass();

    //       expect(data).toEqual({ price: 101, priceReadonly: 201 });
    //     });

    //     it("should reject if condition is not met during updates", async () => {
    //       const toFail = () =>
    //         Book.update(
    //           { bookId: 1, isPublished: false, price: null },
    //           { isPublished: true }
    //         );

    //       expectPromiseFailure(toFail, "Validation Error");

    //       try {
    //         await toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             price: expect.arrayContaining([
    //               "A price is required to publish a book!",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject if condition is not met during updates of readonly", async () => {
    //       const toFail = () => Book.update(book, { price: 101 });

    //       expectPromiseFailure(toFail, "Validation Error");

    //       try {
    //         await toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             priceReadonly: expect.arrayContaining([
    //               "A priceReadonly is required when price is 101!",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should not update callable readonly prop that has changed", async () => {
    //       const toFail = () =>
    //         Book.update(
    //           {
    //             bookId: 1,
    //             isPublished: false,
    //             price: null,
    //             priceReadonly: 201,
    //           },
    //           { priceReadonly: 101 }
    //         );

    //       expectPromiseFailure(toFail, "Nothing to update");
    //     });

    //     it("should accept requiredBy + default(any | function)", () => {
    //       const values = ["", () => ""];

    //       for (const value of values) {
    //         const toPass = fx({
    //           propertyName: {
    //             default: value,
    //             required: () => true,
    //             requiredError: "",
    //             validator,
    //           },
    //         });

    //         expectNoFailure(toPass);

    //         toPass();
    //       }
    //     });

    //     it("should accept requiredBy + readonly", () => {
    //       const toPass = fx({
    //         propertyName: {
    //           default: "",
    //           readonly: true,
    //           required: () => true,
    //           requiredError: "",
    //           validator,
    //         },
    //       });

    //       expectNoFailure(toPass);

    //       toPass();
    //     });

    //     it("should accept requiredBy + requiredError(string | function)", () => {
    //       const values = ["", () => ""];

    //       for (const requiredError of values) {
    //         const toPass = fx({
    //           propertyName: {
    //             default: "",
    //             required: () => true,
    //             requiredError,
    //             validator,
    //           },
    //         });

    //         expectNoFailure(toPass);

    //         toPass();
    //       }
    //     });

    //     it("should reject required(true) + shouldInit", () => {
    //       const values = [false, true, () => "", [], {}];

    //       for (const shouldInit of values) {
    //         const toFail = fx({
    //           propertyName: {
    //             default: "",
    //             required: () => true,
    //             requiredError: "",
    //             shouldInit,
    //             validator,
    //           },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "Required properties cannot have a initialization blocked",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });
    //   });

    //   describe("invalid", () => {
    //     it("should reject requiredBy & no default", () => {
    //       const toFail = fx({
    //         propertyName: {
    //           required: () => true,
    //           requiredError: "",
    //           validator,
    //         },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toMatchObject(
    //           expect.objectContaining({
    //             propertyName: expect.arrayContaining([
    //               "Callable required properties must have a default value or setter",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject requiredError & required(!function)", () => {
    //       const values = [undefined, true, [], {}, false, 2, ""];

    //       for (const required of values) {
    //         const toFail = fx({
    //           propertyName: {
    //             default: true,
    //             required,
    //             requiredError: () => "",
    //             validator,
    //           },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "RequiredError can only be used with a callable required rule",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject requiredBy & no requiredError", () => {
    //       const toFail = fx({
    //         propertyName: {
    //           default: "",
    //           required: () => true,
    //           validator,
    //         },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toMatchObject(
    //           expect.objectContaining({
    //             propertyName: expect.arrayContaining([
    //               "Callable required properties must have a requiredError or setter",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject requiredBy + requiredError(!string & !function)", () => {
    //       const values = [1, {}, [], false, true, undefined];

    //       for (const requiredError of values) {
    //         const toFail = fx({
    //           propertyName: {
    //             default: "",
    //             required: () => true,
    //             requiredError,
    //             validator,
    //           },
    //         });

    //         expectFailure(toFail);

    //         try {
    //           toFail();
    //         } catch (err: any) {
    //           expect(err.payload).toMatchObject(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "RequiredError must be a string or setter",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });

    //     it("should reject requiredBy + default & dependent(true)", () => {
    //       const toFail = fx({
    //         dependentProp: {
    //           default: "value",
    //           dependent: true,
    //           dependsOn: "prop",
    //           resolver: () => 1,
    //           required: () => true,
    //           requiredError: "",
    //           validator,
    //         },
    //         prop: { default: "" },
    //       });

    //       expectFailure(toFail);

    //       try {
    //         toFail();
    //       } catch (err: any) {
    //         expect(err.payload).toMatchObject(
    //           expect.objectContaining({
    //             dependentProp: expect.arrayContaining([
    //               "Required properties cannot be dependent",
    //             ]),
    //           })
    //         );
    //       }
    //     });
    //   });
    // });

    // describe("shouldInit", () => {
    //   describe("valid", () => {
    //     let TestSchema: any;

    //     beforeAll(async () => {
    //       TestSchema = new Schema(
    //         {
    //           env: { default: "dev" },
    //           isBlocked: {
    //             shouldInit: (ctx: any) => ctx.env == "test",
    //             default: false,
    //           },
    //           laxProp: { default: 0 },
    //         },
    //         { errors: "throw" }
    //       ).getModel();
    //     });

    //     it("should respect default rules", async () => {
    //       const { data } = await TestSchema.create({ isBlocked: true });

    //       expect(data).toMatchObject({
    //         env: "dev",
    //         isBlocked: false,
    //         laxProp: 0,
    //       });
    //     });

    //     it("should respect callable should init when condition passes during cloning", async () => {
    //       const { data } = await TestSchema.clone({
    //         env: "test",
    //         isBlocked: "yes",
    //       });

    //       expect(data).toMatchObject({
    //         env: "test",
    //         isBlocked: "yes",
    //         laxProp: 0,
    //       });
    //     });

    //     it("should respect callable should init when condition passes at creation", async () => {
    //       const { data } = await TestSchema.create({
    //         env: "test",
    //         isBlocked: "yes",
    //       });

    //       expect(data).toMatchObject({
    //         env: "test",
    //         isBlocked: "yes",
    //         laxProp: 0,
    //       });
    //     });

    //     it("should accept shouldInit(false) + default", () => {
    //       const fxn = fx({
    //         propertyName: { shouldInit: false, default: true },
    //       });

    //       expectNoFailure(fxn);

    //       fxn();
    //     });

    //     it("should accept shouldInit: () => boolean + default", () => {
    //       const values = [() => true, () => false];

    //       for (const shouldInit of values) {
    //         const fxn = fx({
    //           propertyName: { shouldInit, default: true },
    //         });

    //         expectNoFailure(fxn);

    //         fxn();
    //       }
    //     });
    //   });

    //   describe("invalid", () => {
    //     it("should reject shouldInit(false) & no default", () => {
    //       const fxn = fx({ propertyName: { shouldInit: false } });

    //       expectFailure(fxn);

    //       try {
    //         fxn();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             propertyName: expect.arrayContaining([
    //               "A property with initialization blocked must have a default value",
    //             ]),
    //           })
    //         );
    //       }
    //     });

    //     it("should reject shouldInit !(boolean | () => boolean)", () => {
    //       const values = [undefined, 1, {}, null, [], "yes", "false", "true"];

    //       for (const shouldInit of values) {
    //         const fxn = fx({ propertyName: { shouldInit, default: true } });

    //         expectFailure(fxn);

    //         try {
    //           fxn();
    //         } catch (err: any) {
    //           expect(err.payload).toEqual(
    //             expect.objectContaining({
    //               propertyName: expect.arrayContaining([
    //                 "The initialization of a property can only be blocked if the 'shouldinit' rule is set to 'false' or a function that returns a boolean",
    //               ]),
    //             })
    //           );
    //         }
    //       }
    //     });
    //   });
    // });

    describe("sideEffect", () => {
      describe("valid", () => {
        // const defaultMap = {
        //   sideInit: { hasChanged: false, newValue: undefined },
        //   sideNoInit: { hasChanged: false, newValue: undefined },
        // };
        // let User: any, successMap: any;

        // beforeAll(() => {
        //   User = new Schema(
        //     {
        //       dependentSideNoInit: {
        //         default: "",
        //         dependent: true,
        //         dependsOn: "sideNoInit",
        //         resolver: () => "changed",
        //       },
        //       dependentSideInit: {
        //         default: false,
        //         dependent: true,
        //         dependsOn: "sideInit",
        //         resolver: (ctx: any) => (ctx.sideInit ? true : false),
        //       },
        //       name: { default: "" },
        //       sideInit: {
        //         sideEffect: true,
        //         onSuccess: onSuccess("sideInit"),
        //         validator: validateBoolean,
        //       },
        //       sideNoInit: {
        //         sideEffect: true,
        //         shouldInit: false,
        //         onSuccess: onSuccess("sideNoInit"),
        //         validator: validateBoolean,
        //       },
        //     },
        //     { errors: "throw" }
        //   ).getModel();

        //   function onSuccess(prop: keyof typeof defaultMap) {
        //     return (ctx: any) =>
        //       (successMap[prop] = { hasChanged: true, newValue: ctx[prop] });
        //   }

        //   function validateBoolean(value: any) {
        //     if (![false, true].includes(value))
        //       return { valid: false, reason: `${value} is not a boolean` };
        //     return { valid: true };
        //   }
        // });

        // beforeEach(() => (successMap = { ...defaultMap }));

        // it("should respect sideInits & sideNoInit", async () => {
        //   const { data: user } = await User.create({
        //     sideInit: true,
        //     name: "Peter",
        //   });

        //   expect(user).toEqual({
        //     dependentSideNoInit: "",
        //     dependentSideInit: true,
        //     name: "Peter",
        //   });
        // });

        it("should allow sanitizer", () => {
          const toPass = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: { sideEffect: true, sanitizer: () => "", validator },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow onFailure", () => {
          const toPass = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: {
              sideEffect: true,
              onFailure: validator,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow requiredBy + requiredError", () => {
          const toPass = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: {
              sideEffect: true,
              required: () => true,
              requiredError: () => "",
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow shouldInit(false) + validator", () => {
          const toPass = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: { sideEffect: true, shouldInit: false, validator },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow onSuccess + validator", () => {
          const values = [[], () => ({})];

          for (const onSuccess of values) {
            const toPass = fx({
              dependentProp: {
                default: "",
                dependent: true,
                dependsOn: "propertyName",
                resolver: () => "",
              },
              propertyName: { sideEffect: true, onSuccess, validator },
            });

            expectNoFailure(toPass);

            toPass();
          }
        });

        // describe("onSuccess", () => {
        //   it("should no trigger onSuccess listeners of sideEffects when not provided during cloning", async () => {
        //     const { handleSuccess } = await User.clone({
        //       name: "Peter",
        //       dependentSideNoInit: "",
        //       dependentSideInit: false,
        //     });

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: false, newValue: undefined },
        //       sideNoInit: { hasChanged: false, newValue: undefined },
        //     });
        //   });

        //   it("should no trigger onSuccess listeners of sideEffects when not provided at creation", async () => {
        //     const { handleSuccess } = await User.create({
        //       name: "Peter",
        //       dependentSideNoInit: "",
        //       dependentSideInit: false,
        //     });

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: false, newValue: undefined },
        //       sideNoInit: { hasChanged: false, newValue: undefined },
        //     });
        //   });

        //   it("should no trigger onSuccess listeners of sideEffects when not provided during updates", async () => {
        //     const { handleSuccess } = await User.update(
        //       {
        //         name: "Peter",
        //         dependentSideNoInit: "",
        //         dependentSideInit: false,
        //       },
        //       { name: "John" }
        //     );

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: false, newValue: undefined },
        //       sideNoInit: { hasChanged: false, newValue: undefined },
        //     });
        //   });

        //   it("should respect onSuccess of sideInits & sideNoInit during cloning", async () => {
        //     const sideInit = false;
        //     const { handleSuccess } = await User.clone({
        //       sideInit,
        //       sideNoInit: true,
        //       name: "Peter",
        //       dependentSideNoInit: "",
        //       dependentSideInit: false,
        //     });

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: true, newValue: sideInit },
        //       sideNoInit: { hasChanged: false, newValue: undefined },
        //     });
        //   });

        //   it("should respect onSuccess of sideInits & sideNoInit at creation", async () => {
        //     const sideInit = false;
        //     const { handleSuccess } = await User.create({
        //       sideInit,
        //       sideNoInit: true,
        //       name: "Peter",
        //     });

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: true, newValue: sideInit },
        //       sideNoInit: { hasChanged: false, newValue: undefined },
        //     });
        //   });

        //   it("should respect onSuccess of sideInits & sideNoInit during updates", async () => {
        //     const sideInit = false,
        //       sideNoInit = true;
        //     const { handleSuccess } = await User.update(
        //       {
        //         dependentSideNoInit: "",
        //         dependentSideInit: false,
        //       },
        //       { sideInit, sideNoInit, name: "Peter" }
        //     );

        //     await handleSuccess();

        //     expect(successMap).toEqual({
        //       sideInit: { hasChanged: true, newValue: sideInit },
        //       sideNoInit: { hasChanged: true, newValue: sideNoInit },
        //     });
        //   });
        // });

        // describe("RequiredSideEffect", () => {
        //   let RequiredSideEffect: any;

        //   beforeAll(() => {
        //     RequiredSideEffect = new Schema({
        //       dependent: {
        //         default: "",
        //         dependent: true,
        //         dependsOn: "sideEffect",
        //         resolver: (ctx: any) => ctx.sideEffect,
        //       },
        //       laxProp: { default: "" },
        //       sideEffect: {
        //         sideEffect: true,
        //         required: ({ sideEffect, dependent }: any) => {
        //           return dependent === "" && sideEffect === undefined;
        //         },
        //         requiredError: () => "SideEffect is required",
        //         validator,
        //       },
        //     }).getModel();
        //   });

        //   // creation
        //   it("should create normally", async () => {
        //     const { data } = await RequiredSideEffect.create({
        //       sideEffect: true,
        //       laxProp: "laxProp",
        //     });

        //     expect(data).toEqual({ dependent: true, laxProp: "laxProp" });
        //   });

        //   // cloning
        //   it("should clone normally", async () => {
        //     const { data } = await RequiredSideEffect.clone({
        //       sideEffect: "cloned",
        //       dependent: true,
        //       laxProp: "laxProp",
        //     });

        //     expect(data).toEqual({ dependent: "cloned", laxProp: "laxProp" });
        //   });

        //   it("should require during cloning", async () => {
        //     const { data, error } = await RequiredSideEffect.clone({
        //       dependent: "",
        //       laxProp: "laxProp",
        //     });

        //     expect(data).toBe(undefined);
        //     expect(error).toEqual(
        //       expect.objectContaining({
        //         message: "Validation Error",
        //         payload: { sideEffect: ["SideEffect is required"] },
        //         statusCode: 400,
        //       })
        //     );
        //   });

        //   // updates
        //   it("should update normally", async () => {
        //     const { data } = await RequiredSideEffect.update(
        //       {
        //         dependent: true,
        //         laxProp: "laxProp",
        //       },
        //       { sideEffect: "updated" }
        //     );

        //     expect(data).toEqual({ dependent: "updated" });
        //   });

        //   it("should require during updates", async () => {
        //     const { data, error } = await RequiredSideEffect.update(
        //       {
        //         dependent: "",
        //         laxProp: "laxProp",
        //       },
        //       { laxProp: 2 }
        //     );

        //     expect(data).toBe(undefined);
        //     expect(error).toEqual(
        //       expect.objectContaining({
        //         message: "Validation Error",
        //         payload: { sideEffect: ["SideEffect is required"] },
        //         statusCode: 400,
        //       })
        //     );
        //   });
        // });
      });

      describe("invalid", () => {
        it("should reject invalid sanitizer", () => {
          const values = [-1, 1, true, false, undefined, null, [], {}];

          for (const sanitizer of values) {
            const toFail = fx({
              propertyName: { sideEffect: true, sanitizer, validator },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' must be a function",
                  ]),
                })
              );
            }
          }
        });

        it("should reject 'sanitizer' rule on non-side effects", () => {
          const values = [
            -1,
            1,
            true,
            false,
            undefined,
            null,
            [],
            {},
            () => {},
          ];

          for (const sanitizer of values) {
            const toFail = fx({ propertyName: { default: "", sanitizer } });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "'sanitizer' is only valid on side effects",
                  ]),
                })
              );
            }
          }
        });

        it("should reject sideEffect & no dependent property ", () => {
          const toFail = fx({
            propertyName: { sideEffect: true, validator },
          });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                propertyName: [
                  "A side effect must have atleast one property that depends on it",
                ],
              })
            );
          }
        });

        it("should reject sideEffect & no validator ", () => {
          const toFail = fx({
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: { sideEffect: true },
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
              dependentProp: {
                default: "",
                dependent: true,
                dependsOn: "propertyName",
                resolver: () => "",
              },
              propertyName: { sideEffect: true, shouldInit, validator },
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
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: {
              sideEffect: true,
              shouldInit: false,
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
            dependentProp: {
              default: "",
              dependent: true,
              dependsOn: "propertyName",
              resolver: () => "",
            },
            propertyName: {
              sideEffect: true,
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
            "dependsOn",
            "onCreate",
            "onChange",
            "onDelete",
            "onUpdate",
            "readonly",
            "resolver",
            "value",
          ];

          for (const rule of values) {
            const toFail = fx({
              dependentProp: {
                default: "",
                dependent: true,
                dependsOn: "propertyName",
                resolver: () => "",
              },
              propertyName: {
                sideEffect: true,
                [rule]: true,
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
                    "SideEffects properties can only have (sanitizer, sideEffect, onFailure, onSuccess, required, requiredError, shouldInit, validator) as rules",
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
                    "should be 'silent' or 'throw'",
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
