import { expectFailure, expectNoFailure } from "../_utils";

export const Test_ArchivedSchemas = ({ Schema }: any) => {
  describe("Archived Schemas", () => {
    const bookSchema = new Schema(
      {
        id: { constant: true, value: 1 },
        name: { default: "" },
        price: {
          default: 0,
          dependent: true,
          dependsOn: "_price",
          resolver: () => 100,
        },
        _price: { virtual: true, validator: () => true },
      },
      { timestamps: true }
    );

    describe("behaviour", () => {
      describe("behaviour with no options", () => {
        const Model = bookSchema.getArchivedSchema().getModel();
        const book = { id: 1, name: "Book name", price: 250 };

        it("should create properly", async () => {
          const { data } = Model.create(book);

          expect(data).toEqual(book);
        });
      });
    });

    describe("options", () => {
      const rules = ["onDelete", "onSuccess"];

      describe("valid", () => {
        it("should accept any valid property in archived options", () => {
          const toPass = () =>
            bookSchema.getArchivedSchema({
              archivedAt: "archived_at",
              onDelete: () => {},
              onSuccess: () => {},
            });

          expectNoFailure(toPass);

          toPass();
        });

        it("should not crash if options are not passed to Archived Schema", () => {
          const toPass = () => bookSchema.getArchivedSchema();

          expectNoFailure(toPass);

          toPass();
        });

        it("should accept 'archivedAt' as a string or boolean", () => {
          const values = ["archived_at", true, false];

          for (const archivedAt of values) {
            const toPass = () => bookSchema.getArchivedSchema({ archivedAt });

            expectNoFailure(toPass);

            toPass();
          }
        });

        it("should accept valid 'onDelete' & 'onSuccess' handlers", () => {
          const values = [
            () => {},
            () => ({}),
            [() => {}],
            [() => {}, () => ({})],
          ];

          for (const rule of rules)
            for (const value of values) {
              const toPass = () =>
                bookSchema.getArchivedSchema({
                  archivedAt: "archived_at",
                  [rule]: value,
                });

              expectNoFailure(toPass);

              toPass();
            }
        });
      });

      describe("invalid", () => {
        const ErrorMessage = "Invalid Archived Schema";

        it("should reject options that are not objects", () => {
          const invalidOptions = [-1, 0, 1, null, true, false, [], () => {}];

          for (const option of invalidOptions) {
            const toFail = () => bookSchema.getArchivedSchema(option);

            expectFailure(toFail, ErrorMessage);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ErrorMessage,
                payload: expect.objectContaining({
                  options: ["expected an object"],
                }),
                statusCode: 500,
              });
            }
          }
        });

        it("should reject any invalid property in archived options", () => {
          const errorMessages = [
            "a",
            "b",
            "createdAt",
            "errors",
            "updatedAt",
            "timestamps",
          ].map((prop) => `'${prop}' is not a valid archived option`);

          const toFail = () =>
            bookSchema.getArchivedSchema({
              a: true,
              b: true,
              createdAt: "createdAt",
              errors: "throw",
              updatedAt: "updatedAt",
              timestamps: "throw",
            });

          expectFailure(toFail, ErrorMessage);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: ErrorMessage,
              payload: expect.objectContaining({
                options: errorMessages,
              }),
              statusCode: 500,
            });
          }
        });

        it("should reject 'archivedAt' if not of type string or boolean", () => {
          const values = [{}, [], 1, -1, 0, () => {}, null];

          for (const archivedAt of values) {
            const toFail = () => bookSchema.getArchivedSchema({ archivedAt });

            expectFailure(toFail, ErrorMessage);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ErrorMessage,
                payload: expect.objectContaining({
                  options: expect.arrayContaining([
                    "'archivedAt' should be of type boolean | string",
                  ]),
                }),
                statusCode: 500,
              });
            }
          }
        });

        it("should reject 'archivedAt' if empty string is provided", () => {
          const values = ["", "  "];

          for (const archivedAt of values) {
            const toFail = () => bookSchema.getArchivedSchema({ archivedAt });

            expectFailure(toFail, ErrorMessage);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ErrorMessage,
                payload: expect.objectContaining({
                  options: expect.arrayContaining([
                    "'archivedAt' cannot be an empty string",
                  ]),
                }),
                statusCode: 500,
              });
            }
          }
        });

        it("should reject 'archivedAt' if string provided is a property, virtual or timestamp of parent schema", () => {
          const values = [
            "id",
            "name",
            "price",
            "_price",
            "createdAt",
            "updatedAt",
          ];

          for (const archivedAt of values) {
            const toFail = () => bookSchema.getArchivedSchema({ archivedAt });

            expectFailure(toFail, ErrorMessage);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: ErrorMessage,
                payload: expect.objectContaining({
                  options: expect.arrayContaining([
                    `'${archivedAt}' is a reserved property on your parent schema`,
                  ]),
                }),
                statusCode: 500,
              });
            }
          }
        });

        it("should reject invalid 'onDelete' & 'onSuccess' handlers", () => {
          const values = [1, "", 0, false, true, null, {}];

          for (const rule of rules)
            for (const value of values) {
              const toFail = () =>
                bookSchema.getArchivedSchema({
                  archivedAt: "archived_at",
                  [rule]: value,
                });

              expectFailure(toFail, ErrorMessage);

              try {
                toFail();
              } catch (err: any) {
                expect(err.payload).toEqual(
                  expect.objectContaining({
                    options: expect.arrayContaining([
                      `The '${rule}' handler @[0] is not a function`,
                    ]),
                  })
                );
              }
            }
        });
      });
    });
  });
};
