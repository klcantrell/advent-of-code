open Day5Helpers;

let executeOp = (~codes, ~opAddress, ~params, ~op) => {
  switch (op) {
  | Add =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    Array.concat([
      firstSegment,
      [|params |> List.fold_left((+), 0)|],
      lastSegment,
    ]);

  | Multiply =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    Array.concat([
      firstSegment,
      [|params |> List.fold_left(( * ), 1)|],
      lastSegment,
    ]);

  | Input =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    Array.concat([firstSegment, [|Program.inputPart2|], lastSegment]);

  | Output =>
    print(params |> List.hd);
    codes;

  | LessThan =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let (firstParam, secondParam) = params->toPair;
    if (firstParam < secondParam) {
      Array.concat([firstSegment, [|1|], lastSegment]);
    } else {
      Array.concat([firstSegment, [|0|], lastSegment]);
    };

  | Equals =>
    let (firstSegment, lastSegment) =
      extractOuterSegments(~codes, ~opAddress, ~op);
    let (firstParam, secondParam) = params->toPair;
    if (firstParam == secondParam) {
      Array.concat([firstSegment, [|1|], lastSegment]);
    } else {
      Array.concat([firstSegment, [|0|], lastSegment]);
    };

  | _ => codes
  };
};

let processCodes = codes => {
  let rec traverseAndApplyOp = (codes, opAddress) => {
    let {op, paramModes} = parseInstruction(codes[opAddress]);
    let params = extractInputs(~codes, ~opAddress, ~paramModes, ~op);
    let executeOp = executeOp(~codes, ~opAddress, ~params);
    let nextOpAddress = nextOpAddress(~opAddress, ~op, ~params);
    switch (op) {
    | Done => codes
    | _ => traverseAndApplyOp(executeOp(~op), nextOpAddress)
    };
  };
  traverseAndApplyOp(codes, 0);
};

processCodes(Program.codes);
