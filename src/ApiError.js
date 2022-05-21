module.exports = class ApiError extends Error {
  constructor({ message, payload = {}, statusCode = 400 }) {
    super(message);
    this.payload = payload;
    this.statusCode = statusCode;
  }
};
