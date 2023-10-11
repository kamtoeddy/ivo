export type { Input, Output, EInput, EOutput };

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
