export const schemaDefinition_Tests = ({ Schema }: any) => {
  const fx =
    (definition: any = undefined) =>
    () =>
      new Schema(definition);

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

      for (const value of values) expect(fx(value)).toThrow("Invalid Schema");
    });

    it("should reject if property definitions has no property", () => {
      expect(fx({})).toThrow("Invalid Schema");

      try {
        fx({})();
      } catch (err: any) {
        expect(err.payload).toMatchObject({
          "schema properties": ["Insufficient Schema properties"],
        });
      }
    });

    describe("dependent", () => {
      it("should reject dependent & no default", () => {
        try {
          fx({ age: { dependent: true } })();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              age: expect.arrayContaining([
                "Dependent properties must have a default value",
              ]),
            })
          );
        }
      });

      it("should reject dependent & shouldInit", () => {
        const values = [false, true];

        for (const shouldInit of values)
          try {
            fx({ age: { dependent: true, shouldInit } })();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                age: expect.arrayContaining([
                  "Dependent properties cannot have shouldInit rule",
                ]),
              })
            );
          }
      });

      it("should reject dependent & + required", () => {
        try {
          fx({ age: { dependent: true, required: true } })();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              age: expect.arrayContaining([
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
    //         age: { readonly: "lax", shouldInit: false },
    //       })();
    //     } catch (err: any) {
    //       expect(err.payload).toEqual(
    //         expect.objectContaining({
    //           age: expect.arrayContaining([
    //             "SideEffects must have at least one onChange listener",
    //             "A property should at least be readonly, required, or have a default value",
    //           ]),
    //         })
    //       );
    //     }
    //   });
    // });

    describe("readonly", () => {
      it("should reject readonly & no default", () => {
        try {
          fx({
            age: { readonly: true, shouldInit: false, validator: isNaN },
          })();
        } catch (err: any) {
          expect(err.payload.age).toContain(
            "A property that should not be initialized must have a default value other than 'undefined'"
          );
        }
      });

      it("should reject readonly(lax) & no default", () => {
        try {
          fx({ age: { readonly: "lax", validator: isNaN } })();
        } catch (err: any) {
          expect(err.payload.age).toContain(
            "A property that should not be initialized must have a default value other than 'undefined'"
          );
        }
      });

      it("should reject readonly(lax) & !shouldInit(undefined)", () => {
        const values = [false, true];

        for (const shouldInit of values)
          try {
            fx({ age: { readonly: "lax", shouldInit } })();
          } catch (err: any) {
            expect(err.payload).toEqual(
              expect.objectContaining({
                age: expect.arrayContaining([
                  "lax properties cannot have initialization blocked",
                ]),
              })
            );
          }
      });
    });

    // describe("required", () => {
    //   it("should reject readonly + validator & no default", () => {
    //     try {
    //       fx({
    //         age: { readonly: true, shouldInit: false, validator: isNaN },
    //       })();
    //     } catch (err: any) {
    //       expect(err.payload.age).toContain(
    //         "A property that should not be initialized must have a default value other than 'undefined'"
    //       );
    //     }
    //   });

    //   it("should reject readonly(lax) + validator & no default", () => {
    //     try {
    //       fx({ age: { readonly: "lax", validator: isNaN } })();
    //     } catch (err: any) {
    //       expect(err.payload.age).toContain(
    //         "A property that should not be initialized must have a default value other than 'undefined'"
    //       );
    //     }
    //   });

    //   it("should reject readonly(lax) & !shouldInit(undefined)", () => {
    //     const values = [false, true];

    //     for (const shouldInit of values)
    //       try {
    //         fx({ age: { readonly: "lax", shouldInit } })();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             age: expect.arrayContaining([
    //               "lax properties cannot have initialization blocked",
    //             ]),
    //           })
    //         );
    //       }
    //   });

    //   it("should reject required(ctx)=>boolean & !shouldInit(undefined)", () => {
    //     const values = [false, true];

    //     for (const shouldInit of values)
    //       try {
    //         fx({ age: { readonly: "lax", shouldInit } })();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             age: expect.arrayContaining([
    //               "lax properties cannot have initialization blocked",
    //             ]),
    //           })
    //         );
    //       }
    //   });

    //   it("should reject required(ctx)=>boolean & !shouldInit(undefined)", () => {
    //     const values = [false, true];

    //     for (const shouldInit of values)
    //       try {
    //         fx({ age: { readonly: "lax", shouldInit } })();
    //       } catch (err: any) {
    //         expect(err.payload).toEqual(
    //           expect.objectContaining({
    //             age: expect.arrayContaining([
    //               "lax properties cannot have initialization blocked",
    //             ]),
    //           })
    //         );
    //       }
    //   });
    // });

    describe("sideEffect", () => {
      it("should reject sideEffect & no onChange listeners", () => {
        try {
          fx({
            age: { sideEffect: true, validator: () => ({ valid: true }) },
          })();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              age: expect.arrayContaining([
                "SideEffects must have at least one onChange listener",
                "A property should at least be readonly, required, or have a default value",
              ]),
            })
          );
        }
      });

      it("should reject sideEffect & no validator ", () => {
        try {
          fx({ age: { sideEffect: true, onChange: [() => {}] } })();
        } catch (err: any) {
          expect(err.payload).toEqual(
            expect.objectContaining({
              age: expect.arrayContaining([
                "Invalid validator",
                "A property should at least be readonly, required, or have a default value",
              ]),
            })
          );
        }
      });
    });
  });
};
