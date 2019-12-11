exception InvalidCode;
exception NoPairFound;

let split = Js.String.split;

[@bs.send]
external padStart: (Js.String.t, int, string) => string = "padStart";

let join = Js.Array.joinWith;

let print = Js.log;

type paramMode =
  | Position
  | Immediate;

type ops =
  | Add
  | Multiply
  | Input
  | Output
  | JumpIfTrue
  | JumpIfFalse
  | LessThan
  | Equals
  | Done;

type instruction = {
  op: ops,
  paramModes: list(paramMode),
};

let opOfCode =
  fun
  | 1 => Add
  | 2 => Multiply
  | 3 => Input
  | 4 => Output
  | 5 => JumpIfTrue
  | 6 => JumpIfFalse
  | 7 => LessThan
  | 8 => Equals
  | 99 => Done
  | _ => raise(InvalidCode);

let paramModeOfCode =
  fun
  | 0 => Position
  | 1 => Immediate
  | _ => raise(InvalidCode);

let parseOp = code => {
  let opCodeLength = 2;
  let digits = (code |> string_of_int)->padStart(5, "0") |> split("");
  Array.(
    digits->sub(length(digits) - opCodeLength, opCodeLength)
    |> join("")
    |> int_of_string
    |> opOfCode
  );
};

let parseParams = code => {
  let paramsLength = 3;
  let digits = (code |> string_of_int)->padStart(5, "0") |> split("");
  Array.(
    digits->Array.sub(0, paramsLength)
    |> map(int_of_string)
    |> map(paramModeOfCode)
    |> to_list
  );
};

let toPair = params => {
  (params |> List.hd, params |> List.tl |> List.hd);
};

let instructionSizeOfOp =
  fun
  | Add
  | Multiply
  | LessThan
  | Equals => 4
  | Input
  | Output => 2
  | JumpIfTrue
  | JumpIfFalse => 3
  | _ => 0;

let parseInstruction = code => {
  let op = code |> parseOp;
  let paramModes = code |> parseParams;
  {op, paramModes};
};

let extractInputs = (~codes, ~opAddress, ~paramModes, ~op) => {
  let filteredParamModes =
    switch (op) {
    | Add
    | Multiply
    | JumpIfTrue
    | JumpIfFalse
    | LessThan
    | Equals => paramModes |> List.tl
    | Done => []
    | _ => [paramModes |> List.rev |> List.hd]
    };
  filteredParamModes
  |> List.rev
  |> List.mapi((i, paramMode) => {
       let inputOffset = i + 1;
       switch (paramMode) {
       | Position => codes[codes[opAddress + inputOffset]]
       | Immediate => codes[opAddress + inputOffset]
       };
     });
};

let extractOuterSegments = (~codes, ~opAddress, ~op) => {
  let outputOffset = (op |> instructionSizeOfOp) - 1;
  let outputAddress = codes[opAddress + outputOffset];
  let finalAddress = Array.length(codes) - 1;

  let firstSegment = Array.sub(codes, 0, outputAddress);
  let lastSegment =
    switch (outputAddress) {
    | i when i + 1 == finalAddress => [|codes[finalAddress]|]
    | i when i == finalAddress => [||]
    | _ => Array.sub(codes, outputAddress + 1, finalAddress - outputAddress)
    };

  (firstSegment, lastSegment);
};

let nextOpAddress = (~opAddress, ~op, ~params) => {
  switch (op) {
  | JumpIfTrue =>
    let (firstParam, secondParam) = params->toPair;
    if (firstParam > 0) {
      secondParam;
    } else {
      opAddress + instructionSizeOfOp(op);
    };
  | JumpIfFalse =>
    let (firstParam, secondParam) = params->toPair;
    if (firstParam == 0) {
      secondParam;
    } else {
      opAddress + instructionSizeOfOp(op);
    };
  | _ => opAddress + instructionSizeOfOp(op)
  };
};
