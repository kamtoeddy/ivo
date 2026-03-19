export type { Ctx_Options, EInput, EOutput, Input, Output };

type Ctx_Options = { lang: string };

type Input = {
  firstName: string;
  lastName: string;
};

type Output = {
  firstName: string;
  fullName: string;
  lastName: string;
};

type EInput = {
  firstName: string;
  lastName: string;
};

type EOutput = {
  firstName: string;
  full_name: string;
  lastName: string;
};
