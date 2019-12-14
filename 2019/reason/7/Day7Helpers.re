exception InvalidCode;
exception NoPairFound;
exception MissingStartupArgument;
exception InvalidOutput;
exception InvalidState;

module Dict = {
  type t('a) = Js.Dict.t('a);
  let make = Js.Dict.empty;
  let get = Js.Dict.get;
  let set = Js.Dict.set;
  let keys = Js.Dict.keys;
  let entries = Js.Dict.entries;
  let fromArray = Js.Dict.fromArray;
};

let split = Js.String.split;

[@bs.send]
external padStart: (Js.String.t, int, string) => string = "padStart";

let join = Js.Array.joinWith;

let print = Js.log;

type opResult = {
  codes: array(int),
  output: option(int),
};

type pendingState = {
  address: int,
  state: opResult,
};

type executionStatusSingleNode =
  | Idle
  | Initialized
  | PendingInput(pendingState)
  | Complete(opResult);

type executionStatusMultiNode =
  | Started
  | InProgress
  | Completed;

type gen('a) = unit => option('a);

module type Env = {
  let args: gen(int);
  let startingAddress: int;
};

module type IntCodeComputer = {
  let processCodes: array(int) => executionStatusSingleNode;
};

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

let extractParams = (~codes, ~opAddress, ~paramModes, ~op) => {
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
       let paramOffset = i + 1;
       switch (paramMode) {
       | Position => codes[codes[opAddress + paramOffset]]
       | Immediate => codes[opAddress + paramOffset]
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

let executeOp = (~codes, ~opAddress, ~params, ~op, ~input=99, ~lastOutput, ()) => {
  switch (op) {
  | Add =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let codes =
      Array.concat([
        firstSegment,
        [|params |> List.fold_left((+), 0)|],
        lastSegment,
      ]);
    {codes, output: lastOutput};

  | Multiply =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let codes =
      Array.concat([
        firstSegment,
        [|params |> List.fold_left(( * ), 1)|],
        lastSegment,
      ]);
    {codes, output: lastOutput};

  | Input =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let codes = Array.concat([firstSegment, [|input|], lastSegment]);
    {codes, output: lastOutput};

  | Output =>
    let output = params |> List.hd;
    {codes, output: Some(output)};

  | LessThan =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let (firstParam, secondParam) = params->toPair;
    if (firstParam < secondParam) {
      let codes = Array.concat([firstSegment, [|1|], lastSegment]);
      {codes, output: lastOutput};
    } else {
      let codes = Array.concat([firstSegment, [|0|], lastSegment]);
      {codes, output: lastOutput};
    };

  | Equals =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let (firstParam, secondParam) = params->toPair;
    if (firstParam == secondParam) {
      let codes = Array.concat([firstSegment, [|1|], lastSegment]);
      {codes, output: lastOutput};
    } else {
      let codes = Array.concat([firstSegment, [|0|], lastSegment]);
      {codes, output: lastOutput};
    };

  | _ => {codes, output: lastOutput}
  };
};

let rec generateCombinations = initialCombination => {
  let rec make = (~combinations=[||], ~index=0, ()) =>
    if (Array.length(initialCombination) == 1) {
      [|initialCombination|];
    } else if (index < Array.length(initialCombination)) {
      let first = initialCombination[index];
      let finalIndex = Array.length(initialCombination) - 1;
      let rest =
        switch (index) {
        | i when i + 1 == finalIndex =>
          Array.(
            concat([
              sub(initialCombination, 0, index),
              [|initialCombination[finalIndex]|],
            ])
          )
        | i when i == finalIndex =>
          Array.sub(initialCombination, 0, finalIndex)
        | _ =>
          Array.(
            concat([
              sub(initialCombination, 0, index),
              sub(initialCombination, index + 1, finalIndex - index),
            ])
          )
        };
      let innerCombinations =
        rest
        |> generateCombinations
        |> Array.map(p => Array.concat([[|first|], p]));
      make(
        ~combinations=Array.concat([combinations, innerCombinations]),
        ~index=index + 1,
        (),
      );
    } else {
      combinations;
    };
  make();
};

let genOfList: list('a) => gen('a) =
  l => {
    let current = ref(l);
    () =>
      switch (current^) {
      | [] => None
      | [head, ...tail] =>
        current := tail;
        Some(head);
      };
  };

let executionStatusOfProgramState = programState => {
  programState
  |> Dict.keys
  |> Array.fold_left(
       (status, ampNum) => {
         switch (status) {
         | InProgress => InProgress
         | Started
         | Completed =>
           switch (programState->Dict.get(ampNum)) {
           | None => raise(InvalidState)
           | Some(state) =>
             switch (state) {
             | Complete(_) => Completed
             | _ => InProgress
             }
           }
         }
       },
       Started,
     );
};

let initializationStatusOfProgramState = programState =>
  programState
  |> Dict.keys
  |> (ampNums => Array.length(ampNums) == 0 ? Idle : Initialized);

let copyProgramState = programState =>
  programState |> Dict.entries |> Dict.fromArray;

let nodeOutputOfProgramState = (programState, nodeNum) => {
  switch (programState->Dict.get(nodeNum |> string_of_int)) {
  | None => raise(InvalidState)
  | Some(prevState) =>
    switch (prevState) {
    | PendingInput({state: {output: prevOutput}})
    | Complete({output: prevOutput}) =>
      switch (prevOutput) {
      | None => raise(InvalidState)
      | Some(result) => result
      }
    | _ => raise(InvalidState)
    }
  };
};

let nodeStateOfProgramState = (programState, nodeNum) => {
  switch (programState->Dict.get(nodeNum |> string_of_int)) {
  | None => raise(InvalidState)
  | Some(stateToResume) =>
    switch (stateToResume) {
    | PendingInput({state: {codes}, address}) => (codes, address)
    | _ => raise(InvalidState)
    }
  };
};

module IntCodeComputer = {
  module Make = (Env: Env) : IntCodeComputer => {
    let nextArg = Env.args;
    let initialOpAddress = Env.startingAddress;
    let initialOutput = None;

    let processCodes = codes => {
      let rec traverseAndApplyOp = (opResult, opAddress) => {
        let {op, paramModes} = parseInstruction(codes[opAddress]);
        let params =
          extractParams(~codes=opResult.codes, ~opAddress, ~paramModes, ~op);
        let executeOp =
          executeOp(
            ~codes=opResult.codes,
            ~opAddress,
            ~params,
            ~lastOutput=opResult.output,
          );
        let nextOpAddress = nextOpAddress(~opAddress, ~op, ~params);
        switch (op) {
        | Done => Complete(opResult)
        | Input =>
          switch (nextArg()) {
          | None => PendingInput({state: opResult, address: opAddress})
          | Some(arg) =>
            traverseAndApplyOp(executeOp(~op, ~input=arg, ()), nextOpAddress)
          }
        | _ => traverseAndApplyOp(executeOp(~op, ()), nextOpAddress)
        };
      };
      traverseAndApplyOp({codes, output: initialOutput}, initialOpAddress);
    };
  };
};

let computeInitialProgramState = (phaseConfig, prevState) => {
  phaseConfig
  |> Array.mapi((nodeNum, phaseSetting) => (nodeNum, phaseSetting))
  |> Array.fold_left(
       (nextState, (nodeNum, phaseSetting)) => {
         let input =
           if (nodeNum == 0) {
             AmpProgram.initialInput;
           } else {
             let previousNode = nodeNum - 1;
             nodeOutputOfProgramState(nextState, previousNode);
           };
         module IntCodeComputer =
           IntCodeComputer.Make({
             let args = genOfList([phaseSetting, input]);
             let startingAddress = 0;
           });
         nextState->Dict.set(
           nodeNum |> string_of_int,
           IntCodeComputer.processCodes(AmpProgram.codes),
         );
         nextState;
       },
       prevState,
     );
};

let computeNextProgramState = prevState => {
  prevState
  |> Dict.keys
  |> Array.map(int_of_string)
  |> Array.fold_left(
       (nextState, nodeNum) => {
         let previousNode = nodeNum == 0 ? 4 : nodeNum - 1;
         let input = nodeOutputOfProgramState(nextState, previousNode);
         let (codes, address) = nodeStateOfProgramState(nextState, nodeNum);
         module IntCodeComputer =
           IntCodeComputer.Make({
             let args = genOfList([input]);
             let startingAddress = address;
           });
         nextState->Dict.set(
           nodeNum |> string_of_int,
           IntCodeComputer.processCodes(codes),
         );
         nextState;
       },
       prevState,
     );
};
