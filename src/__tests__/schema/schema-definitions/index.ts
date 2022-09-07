export const schemaDefinition_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined, options: any = { timestamps: false }) =>
    () =>
      new Schema(definition, options);

  const expectFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).toThrow(message);
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

    describe("dependent", () => {
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
                "Dependent properties cannot be required",
              ]),
            })
          );
        }
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
                  "Lax properties must have a default value nor setter",
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

    describe("readonly", () => {
      describe("valid", () => {
        it("should allow readonly(true) + dependent + default", () => {
          const toPass = fx({
            propertyName: { readonly: true, dependent: true, default: "" },
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
                    "Strictly readonly properties are required. Remove the required rule",
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
                    "Strictly required properties cannot be dependent",
                  ]),
                })
              );
            }
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
        it("should allow onChange + validator", () => {
          const toPass = fx({
            propertyName: { sideEffect: true, onChange: validator, validator },
          });

          expectNoFailure(toPass);

          toPass();
        });

        it("should allow onChange + shouldInit + validator", () => {
          const toPass = fx({
            propertyName: {
              sideEffect: true,
              shouldInit: true,
              onChange: validator,
              validator,
            },
          });

          expectNoFailure(toPass);

          toPass();
        });
      });

      describe("valid", () => {
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

            entity = await Model(inputValue).create();
          });

          it("should populate createdAt & updatedAt at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).toHaveProperty("createdAt");
            expect(entity).toHaveProperty("updatedAt");
          });

          it("should populate createdAt & updatedAt during cloning", async () => {
            const clone = await Model(entity).clone({ reset: "propertyName2" });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).toHaveProperty("createdAt");
            expect(clone).toHaveProperty("updatedAt");
          });

          it("should populate updatedAt during updates", async () => {
            const updates = await Model(entity).update({ propertyName2: 20 });

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

            entity = await Model(inputValue).create();
          });

          it("should populate c_At & updatedAt at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("createdAt");
            expect(entity).toHaveProperty("c_At");
            expect(entity).toHaveProperty("updatedAt");
          });

          it("should populate c_At & updatedAt during cloning", async () => {
            const clone = await Model(entity).clone({ reset: "propertyName2" });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).not.toHaveProperty("createdAt");
            expect(clone).toHaveProperty("c_At");
            expect(clone).toHaveProperty("updatedAt");
          });

          it("should populate updatedAt during updates", async () => {
            const updates = await Model(entity).update({ propertyName2: 20 });

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

            entity = await Model(inputValue).create();
          });

          it("should populate createdAt & u_At at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("updatedAt");
            expect(entity).toHaveProperty("createdAt");
            expect(entity).toHaveProperty("u_At");
          });

          it("should populate createdAt & u_At during cloning", async () => {
            const clone = await Model(entity).clone({ reset: "propertyName2" });

            expect(clone).toMatchObject({
              propertyName1: "value1",
              propertyName2: "",
            });

            expect(clone).not.toHaveProperty("updatedAt");
            expect(clone).toHaveProperty("createdAt");
            expect(clone).toHaveProperty("u_At");
          });

          it("should populate u_At during updates", async () => {
            const updates = await Model(entity).update({ propertyName2: 20 });

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

            entity = await Model(inputValue).create();
          });

          it("should populate c_At & u_At at creation", () => {
            expect(entity).toMatchObject(inputValue);

            expect(entity).not.toHaveProperty("createdAt");
            expect(entity).not.toHaveProperty("updatedAt");
            expect(entity).toHaveProperty("c_At");
            expect(entity).toHaveProperty("u_At");
          });

          it("should populate c_At & u_At during cloning", async () => {
            const clone = await Model(entity).clone({ reset: "propertyName2" });

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
            const updates = await Model(entity).update({ propertyName2: 20 });

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
