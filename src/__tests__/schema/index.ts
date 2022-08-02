import { Schema } from "../../schema";
import { makeModel } from "../../schema/model";
import { IUser } from "./interface";
import { onNameChange, validate_String } from "./validators";

const userSchema = new Schema(
  {
    dob: { required: true, validator: validate_String("Invalid dob") },
    firstName: {
      required: true,
      onCreate: [onNameChange],
      onUpdate: [onNameChange],
      validator: validate_String("Invalid firstName"),
    },
    id: { readonly: true, validator: validate_String("Invalid id") },
    lastName: {
      required: true,
      onCreate: [onNameChange],
      onUpdate: [onNameChange],
      validator: validate_String("Invalid lastName"),
    },
    name: { default: "", dependent: true },
  },
  { timestamps: true }
);

const UserModel = makeModel<IUser>(userSchema);

export { UserModel };
