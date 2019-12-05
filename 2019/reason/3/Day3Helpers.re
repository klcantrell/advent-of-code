exception UnknownDirection;
exception NoMatchingPositionFound;

type tracing =
  | Up(int)
  | Right(int)
  | Down(int)
  | Left(int);

type tracingStep =
  | GoUp
  | GoRight
  | GoDown
  | GoLeft;

type coordinateData = {
  point: (int, int),
  steps: int,
};

let split = Js.String.split;
let join = Js.Array.joinWith;
let print = Js.log;

let tracingOfString = str => {
  let [direction, ...stepValueDigits] = str |> split("") |> Array.to_list;
  let traceDistance =
    stepValueDigits |> Array.of_list |> join("") |> int_of_string;
  switch (direction) {
  | "U" => Up(traceDistance)
  | "R" => Right(traceDistance)
  | "D" => Down(traceDistance)
  | "L" => Left(traceDistance)
  | _ => raise(UnknownDirection)
  };
};

let stepsOfTracing =
  fun
  | Up(amount) => GoUp |> Array.make(amount) |> Array.to_list
  | Right(amount) => GoRight |> Array.make(amount) |> Array.to_list
  | Down(amount) => GoDown |> Array.make(amount) |> Array.to_list
  | Left(amount) => GoLeft |> Array.make(amount) |> Array.to_list;

let newCoordinate = (coordinate, tracingStep) => {
  let (x, y) = coordinate;
  switch (tracingStep) {
  | GoUp => (x, y + 1)
  | GoRight => (x + 1, y)
  | GoDown => (x, y - 1)
  | GoLeft => (x - 1, y)
  };
};

let extractTracingSteps = traceData => {
  traceData
  |> List.map(tracingOfString)
  |> List.map(stepsOfTracing)
  |> List.flatten;
};

let initializePath = instructionsLength =>
  (instructionsLength + 1)->Array.make((0, 0));

let createPath = instructions => {
  let path = initializePath(instructions->List.length);
  instructions
  |> List.iteri((idx, instruction) => {
       let idxToSet = idx + 1;
       switch (instruction) {
       | GoUp => path[idxToSet] = newCoordinate(path[idx], GoUp)
       | GoRight => path[idxToSet] = newCoordinate(path[idx], GoRight)
       | GoDown => path[idxToSet] = newCoordinate(path[idx], GoDown)
       | GoLeft => path[idxToSet] = newCoordinate(path[idx], GoLeft)
       };
     });
  path;
};

let rec findIntersectingCoordinates =
        (
          ~wireOnePath,
          ~wireTwoPath,
          ~wireOneTracePosition=1,
          ~wireTwoTracePosition=1,
          ~coordinates=[],
          (),
        ) => {
  let wireOneLength = Array.length(wireOnePath);
  let wireTwoLength = Array.length(wireTwoPath);
  switch (wireOneTracePosition, wireTwoTracePosition) {
  | _ when wireOneTracePosition == wireOneLength => coordinates
  | _ when wireTwoTracePosition == wireTwoLength =>
    findIntersectingCoordinates(
      ~wireOnePath,
      ~wireTwoPath,
      ~wireOneTracePosition=wireOneTracePosition + 1,
      ~coordinates,
      (),
    )
  | _ =>
    if (wireOnePath[wireOneTracePosition] == wireTwoPath[wireTwoTracePosition]) {
      findIntersectingCoordinates(
        ~wireOnePath,
        ~wireTwoPath,
        ~wireOneTracePosition,
        ~wireTwoTracePosition=wireTwoTracePosition + 1,
        ~coordinates=
          List.append(
            coordinates,
            [
              {
                point: wireOnePath[wireOneTracePosition],
                steps: wireOneTracePosition + wireTwoTracePosition,
              },
            ],
          ),
        (),
      );
    } else {
      findIntersectingCoordinates(
        ~wireOnePath,
        ~wireTwoPath,
        ~wireOneTracePosition,
        ~wireTwoTracePosition=wireTwoTracePosition + 1,
        ~coordinates,
        (),
      );
    }
  };
};

let distanceOfCoordinate = ((x, y)) => {
  abs(x) + abs(y);
};

let orderByDistanceAscending = ({point: left}, {point: right}) => {
  let leftDistance = distanceOfCoordinate(left);
  let rightDistance = distanceOfCoordinate(right);
  switch (leftDistance, rightDistance) {
  | _ when leftDistance < rightDistance => (-1)
  | _ when leftDistance > rightDistance => 1
  | _ => 0
  };
};

let orderByStepsAscending = ({steps: left}, {steps: right}) => {
  switch (left, right) {
  | _ when left < right => (-1)
  | _ when left > right => 1
  | _ => 0
  };
};
