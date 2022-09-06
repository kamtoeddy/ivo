export const schemaDefinition_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined) =>
    () =>
      new Schema(definition);

  const expectFailure = (fx: Function, message = "Invalid Schema") => {
    expect(fx).toThrow(message);
  };

  const expectNoFailure = (fx: Function) => {
    expect(fx).not.toThrow();
  };

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
          const toFail = fx({ propertyName: { dependent: true, shouldInit } });

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
          propertyName: { dependent: true, required: true },
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

    // describe("lax props", () => {
    //   it("should reject readonly(lax) & shouldInit(false)", () => {
    //     try {
    //       fx({
    //         propertyName: { readonly: "lax", shouldInit: false },
    //       })();
    //     } catch (err: any) {
    //       expect(err.payload).toEqual(
    //         expect.objectContaining({
    //           propertyName: expect.arrayContaining([
    //             "SideEffects must have at least one onChange listener",
    //             "A property should at least be readonly, required, or have a default value",
    //           ]),
    //         })
    //       );
    //     }
    //   });
    // });

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
                    "readonly properties are required by default",
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
                "readonly properties must have a default value or a default setter",
              ]),
            });
          }
        });

        it("should reject readonly(lax) & !shouldInit(undefined)", () => {
          const values = [false, true];

          for (const shouldInit of values) {
            const toFail = fx({
              propertyName: { readonly: "lax", shouldInit },
            });

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err.payload).toEqual(
                expect.objectContaining({
                  propertyName: expect.arrayContaining([
                    "lax properties cannot have initialization blocked",
                  ]),
                })
              );
            }
          }
        });
      });
    });

    describe("required", () => {
      // it("should reject required & no validator", () => {
      //   const toFail = fx({ propertyName: { required: true } });
      //   expectFailure(toFail);
      //   try {
      //     toFail();
      //   } catch (err: any) {
      //     expect(err.payload).toEqual(
      //       expect.objectContaining({
      //         propertyName: ["Required properties must have a validator"],
      //       })
      //     );
      //   }
      // });
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
      it("should reject sideEffect & no onChange listeners", () => {
        const toFail = fx({ propertyName: { sideEffect: true } });

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
        const toFail = fx({ propertyName: { sideEffect: true } });

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
};
