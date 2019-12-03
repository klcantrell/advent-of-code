open Jest;
open Expect;
open Day1;

describe("Day1", () => {
  test("should calculate the fuel for a single module's mass", () => {
    let actual = (fuelOfMass(1969), fuelOfMass(14));
    let expected = (654, 2);
    expect(actual) |> toEqual(expected);
  });
  test("should calculate the fuel for a list of modules using part 1's requirements", () =>
    expect(calculateFuelPart1([1969, 14])) |> toBe(656)
  );
  test("should calculate the fuel for a list of modules using part 2's requirements", () => 
    expect(calculateFuelPart2([100756])) |> toBe(50346)
  );
});
