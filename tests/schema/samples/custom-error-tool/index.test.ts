import { describe, expect, it } from "bun:test";

import { VALIDATION_ERRORS } from "../../../../dist";

import { UserModel, EUserModel } from ".";

describe("Custom ErrorTool", () => {
  describe("Base Model", () => {
    it("should return custom errors with Model.create", async () => {
      const firstName = "name",
        lastName = "";

      const { data, error } = await UserModel.create({ firstName, lastName });

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.VALIDATION_ERROR,
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: "firstName",
            value: firstName,
            message: "Invalid first name",
            metadata: { hint: "try harder 😜", valueProvided: firstName },
          }),
          expect.objectContaining({
            field: "lastName",
            value: lastName,
            message: "validation failed",
          }),
        ]),
      });
    });

    it("should return custom errors with Model.update", async () => {
      const firstName = "name",
        lastName = "";

      const { data, error } = await UserModel.update(
        { firstName: "John", fullName: "", lastName: "doe" },
        { firstName, lastName },
      );

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.VALIDATION_ERROR,
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: "firstName",
            value: firstName,
            message: "Invalid first name",
            metadata: { hint: "try harder 😜", valueProvided: firstName },
          }),
          expect.objectContaining({
            field: "lastName",
            value: lastName,
            message: "validation failed",
          }),
        ]),
      });
    });

    it("should return nothing to update with Model.update when nessecary", async () => {
      const firstName = "name",
        lastName = "lname";

      const { data, error } = await UserModel.update(
        { firstName, fullName: `${firstName} ${lastName}`, lastName },
        { firstName, lastName },
      );

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.NOTHING_TO_UPDATE,
        errors: [],
      });
    });
  });

  describe("Extended Model", () => {
    it("should return custom errors with Model.create", async () => {
      const firstName = "name",
        lastName = "";

      const { data, error } = await EUserModel.create({
        firstName,
        lastName,
      });

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.VALIDATION_ERROR,
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: "firstName",
            value: firstName,
            message: "Invalid first name",
            metadata: { hint: "try harder 😜", valueProvided: firstName },
          }),
          expect.objectContaining({
            field: "lastName",
            value: lastName,
            message: "validation failed",
          }),
        ]),
      });
    });

    it("should return custom errors with Model.update", async () => {
      const firstName = "name",
        lastName = "";

      const { data, error } = await EUserModel.update(
        { firstName: "John", full_name: "", lastName: "doe" },
        { firstName, lastName },
      );

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.VALIDATION_ERROR,
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: "firstName",
            value: firstName,
            message: "Invalid first name",
            metadata: { hint: "try harder 😜", valueProvided: firstName },
          }),
          expect.objectContaining({
            field: "lastName",
            value: lastName,
            message: "validation failed",
          }),
        ]),
      });
    });

    it("should return nothing to update with Model.update when nessecary", async () => {
      const firstName = "name",
        lastName = "lname";

      const { data, error } = await EUserModel.update(
        { firstName, full_name: `${firstName} ${lastName}`, lastName },
        { firstName, lastName },
      );

      expect(data).toBeNull();
      expect(error).toMatchObject({
        message: VALIDATION_ERRORS.NOTHING_TO_UPDATE,
        errors: [],
      });
    });
  });
});
