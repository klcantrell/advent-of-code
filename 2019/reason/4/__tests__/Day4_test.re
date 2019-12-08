open Jest;
open Expect;
open Day4;
open Day4Helpers;

type testCase = {
  password: int,
  expected: verdict,
};

describe("Day4", () => {
  testAll(
    "should satisfy part 1 requirements",
    [
      {password: 111111, expected: GoodToGo},
      {password: 123444, expected: GoodToGo},
      {password: 123789, expected: YouShallNotPass},
    ],
    ({password, expected}) => {
      let result = satisfyRequirementsPart1(password |> digitsOfPassword);
      expect(result) |> toBe(expected);
    },
  );

  testAll(
    "should satisfy part 2 requirements",
    [
      {password: 111111, expected: YouShallNotPass},
      {password: 123444, expected: YouShallNotPass},
      {password: 111122, expected: GoodToGo},
    ],
    ({password, expected}) => {
      let result = satisfyRequirementsPart2(password |> digitsOfPassword);
      expect(result) |> toBe(expected);
    },
  );
});
