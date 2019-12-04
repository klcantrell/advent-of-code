open Day2Helpers;

let processCodes = codes => {
  let rec traverseAndApplyOps = (codes, opPosition) => {
    let nextOpPosition = opPosition + 4;
    switch (codes[opPosition] |> opOfCode) {
    | Add => traverseAndApplyOps(doAddOp(codes, opPosition), nextOpPosition)
    | Multiply =>
      traverseAndApplyOps(doMultiplyOp(codes, opPosition), nextOpPosition)
    | Done => codes
    };
  };
  traverseAndApplyOps(codes, 0);
};

let rec findMagicNounVerbPair = (~noun=0, ~verb=0, ~limit=99, ()) => {
  switch (noun, verb) {
  | (noun, _) when noun == limit => None
  | (_, verb) when verb == limit => findMagicNounVerbPair(~noun=noun + 1, ())
  | pair =>
    let output =
      (InitialCodes.values |> withNounVerbPair(pair) |> processCodes)
      ->Array.get(0);
    if (output == 19690720) {
      Some(pair);
    } else {
      findMagicNounVerbPair(~noun, ~verb=verb + 1, ());
    };
  };
};

let magicNounVerbPairCode =
  switch (findMagicNounVerbPair()) {
  | None => raise(NoPairFound)
  | Some((noun, verb)) => 100 * noun + verb
  };
