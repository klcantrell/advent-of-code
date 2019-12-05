open Jest;
open Expect;
open Day3;

type testCase = {
  input: (list(string), list(string)),
  expectedLowestDistance: int,
  expectedFewestSteps: int,
};

let testCases = [
  {
    input: (
      ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
      ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    ),
    expectedLowestDistance: 159,
    expectedFewestSteps: 610,
  },
  {
    input: (
      [
        "R98",
        "U47",
        "R26",
        "D63",
        "R33",
        "U87",
        "L62",
        "D20",
        "R33",
        "U53",
        "R51",
      ],
      ["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
    ),
    expectedLowestDistance: 135,
    expectedFewestSteps: 410,
  },
];

describe("Day3", () => {
  testAll(
    "should find the distance of the closest intersecting coordinates",
    testCases,
    ({input, expectedLowestDistance: expected}) => {
      let (wireOne, wireTwo) = input;
      let actual = distanceToClosestIntersectingCoordinate(wireOne, wireTwo);
      expect(actual) |> toBe(expected);
    },
  );

  testAll(
    "should find the fewest combined steps to reach an intersection",
    testCases,
    ({input, expectedFewestSteps: expected}) => {
      let (wireOne, wireTwo) = input;
      let actual = fewestStepsToIntersectingCoordinate(wireOne, wireTwo);
      expect(actual) |> toBe(expected);
    },
  );
});
