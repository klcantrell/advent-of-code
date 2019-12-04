exception InvalidCode;
exception NoPairFound;

type ops =
  | Add
  | Multiply
  | Done;

let opOfCode =
  fun
  | 1 => Add
  | 2 => Multiply
  | 99 => Done
  | _ => raise(InvalidCode);

type opComponents = {
  firstSegment: array(int),
  lastSegment: array(int),
  firstInput: int,
  secondInput: int,
};

let extractOpComponents = (codes, position) => {
  let firstInput = codes[codes[position + 1]];
  let secondInput = codes[codes[position + 2]];
  let outputPosition = codes[position + 3];
  let finalPosition = Array.length(codes) - 1;

  let firstSegment = Array.sub(codes, 0, outputPosition);
  let lastSegment =
    switch (outputPosition) {
    | i when i + 1 == finalPosition => [|codes[finalPosition]|]
    | i when i == finalPosition => [||]
    | _ =>
      Array.sub(codes, outputPosition + 1, finalPosition - outputPosition)
    };

  {firstSegment, lastSegment, firstInput, secondInput};
};

let doAddOp = (codes, position) => {
  let {firstSegment, lastSegment, firstInput, secondInput} =
    extractOpComponents(codes, position);

  Array.concat([firstSegment, [|firstInput + secondInput|], lastSegment]);
};

let doMultiplyOp = (codes, position) => {
  let {firstSegment, lastSegment, firstInput, secondInput} =
    extractOpComponents(codes, position);

  Array.concat([firstSegment, [|firstInput * secondInput|], lastSegment]);
};

let withNounVerbPair = (pair, codes) => {
  let (noun, verb) = pair;
  let startOfLastSegment = 3;
  Array.(
    concat([
      [|codes[0]|],
      [|noun, verb|],
      sub(codes, startOfLastSegment, length(codes) - startOfLastSegment),
    ])
  );
};
