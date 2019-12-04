open Jest;
open Expect;
open Day2;

type testCase = {
  input: array(int),
  expected: array(int),
};

describe("Day2", () =>
  testAll(
    "should process codes according to the challenge requirements",
    [
      {input: [|1, 0, 0, 0, 99|], expected: [|2, 0, 0, 0, 99|]},
      {
        input: [|1, 1, 1, 4, 99, 5, 6, 0, 99|],
        expected: [|30, 1, 1, 4, 2, 5, 6, 0, 99|],
      },
    ],
    ({input, expected}) =>
    expect(processCodes(input)) |> toEqual(expected)
  )
);
