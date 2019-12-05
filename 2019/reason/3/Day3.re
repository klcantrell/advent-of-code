open Day3Helpers;

let findIntersectingCoordinates = (wireOneTracingInput, wireTwoTracingInput) => {
  let wireOneTracingSteps = extractTracingSteps(wireOneTracingInput);
  let wireTwoTracingSteps = extractTracingSteps(wireTwoTracingInput);
  let wireOnePath = createPath(wireOneTracingSteps);
  let wireTwoPath = createPath(wireTwoTracingSteps);

  findIntersectingCoordinates(~wireOnePath, ~wireTwoPath, ());
};

let distanceToClosestIntersectingCoordinate =
    (wireOneTracingInput, wireTwoTracingInput) => {
  let coordinate =
    findIntersectingCoordinates(wireOneTracingInput, wireTwoTracingInput)
    |> List.sort(orderByDistanceAscending)
    |> List.hd;
  coordinate.point |> distanceOfCoordinate;
};

let fewestStepsToIntersectingCoordinate =
    (wireOneTracingInput, wireTwoTracingInput) => {
  let coordinate =
    findIntersectingCoordinates(wireOneTracingInput, wireTwoTracingInput)
    |> List.sort(orderByStepsAscending)
    |> List.hd;
  coordinate.steps;
};

print(
  distanceToClosestIntersectingCoordinate(
    Wires.First.values,
    Wires.Second.values,
  ),
);

print(
  fewestStepsToIntersectingCoordinate(
    Wires.First.values,
    Wires.Second.values,
  ),
);
