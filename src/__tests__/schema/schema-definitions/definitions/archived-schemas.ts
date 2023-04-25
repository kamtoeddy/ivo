import { expectFailure, expectNoFailure } from "../_utils";

export const Test_ArchivedSchemas = ({ Schema }: any) => {
  describe("Archived Schemas", () => {
    const bookSchema = new Schema({
      id: { constant: true, value: 1 },
      name: { default: "" },
      price: {
        default: 0,
        dependent: true,
        dependsOn: "_price",
        resolver: () => 100,
      },
      _price: { virtual: true, validator: () => true },
    });

    describe("options", () => {
      describe("valid", () => {
        it("should accept any valid property in archived options", () => {
          const toPass = () =>
            bookSchema.getArchivedSchema({
              createdAt: "archivedAt",
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
      });

      describe("invalid", () => {
        it("should reject options that are not objects", () => {
          const invalidOptions = [-1, 0, 1, null, true, false, [], () => {}];

          for (const option of invalidOptions) {
            const toFail = () => bookSchema.getArchivedSchema(option);

            expectFailure(toFail);

            try {
              toFail();
            } catch (err: any) {
              expect(err).toMatchObject({
                message: "Invalid Schema",
                payload: expect.objectContaining({
                  options: ["expected an object"],
                }),
                statusCode: 500,
              });
            }
          }
        });

        it("should reject any invalid property in archived options", () => {
          const errorMessages = ["a", "b", "errors", "timestamps"].map(
            (prop) => `'${prop}' is not a valid archived option`
          );

          const toFail = () =>
            bookSchema.getArchivedSchema({
              a: true,
              b: true,
              errors: "throw",
              timestamps: "throw",
            });

          expectFailure(toFail);

          try {
            toFail();
          } catch (err: any) {
            expect(err).toMatchObject({
              message: "Invalid Schema",
              payload: expect.objectContaining({
                options: errorMessages,
              }),
              statusCode: 500,
            });
          }
        });
      });
    });
  });
};
