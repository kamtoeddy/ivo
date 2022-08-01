import { UserModel } from ".";
import { IUser } from "./interface";

describe("Testing User Schema", () => {
  let user: IUser;

  beforeAll(async () => {
    user = await UserModel({
      id: "1",
      dob: "yesterday",
      firstName: " John",
      lastName: "Smith ",
    }).create();

    // console.log(user);
  });

  it("should initialise with properties assigned", () => {
    expect(user).toMatchObject<IUser>({
      id: "1",
      dob: "yesterday",
      firstName: "John",
      lastName: "Smith",
    });
  });

  it("should initialise and set values of dependent properties", () => {
    expect(user).toMatchObject<IUser>({ name: "John Smith" });
  });
});
