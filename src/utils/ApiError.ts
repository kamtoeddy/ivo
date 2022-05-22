import { looseObject } from "./interfaces";

type apiErrorProps = {
  message: string;
  payload?: looseObject;
  statusCode?: number;
};

export default class ApiError extends Error {
  payload?: looseObject;
  statusCode: number;

  constructor({ message, payload = {}, statusCode = 400 }: apiErrorProps) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }
}
