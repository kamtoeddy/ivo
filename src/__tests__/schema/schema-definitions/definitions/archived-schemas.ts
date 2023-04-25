import { expectFailure } from "../_utils";

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
      });
    });
  });
};
