module Dict = {
  type t('a) = Js.Dict.t('a);
  let make = Js.Dict.empty;
  let get = Js.Dict.get;
  let set = Js.Dict.set;
  let keys = Js.Dict.keys;
};

type verdict =
  | YouShallNotPass
  | GoodToGo;

type adjacentRepetition = {
  digit: int,
  count: int,
};

type hasAdjacentRepetitions =
  | NoMatch(int)
  | Match(Dict.t(adjacentRepetition), int);

let bind = (a, andThen) =>
  switch (a) {
  | None => None
  | Some(a') => andThen(a')
  };

let (>>=) = bind;

let split = Js.String.split;

let print = Js.log;

let digitsOfPassword = password =>
  password
  |> string_of_int
  |> split("")
  |> Array.map(digit => int_of_string(digit));

let withinRepetitionTolerancePart1 = count => count >= 2;
let withinRepetitionTolerancePart2 = count => count == 2;

let findAdjacentRepetitions = password => {
  let [head, ...tail] = password |> Array.to_list;
  let adjacentRepetitions =
    tail
    |> List.fold_left(
         (verdict, digit) => {
           switch (verdict) {
           | NoMatch(prevDigit) =>
             if (digit == prevDigit) {
               let adjacentRepetitions = Dict.make();
               let digitKey = string_of_int(digit);
               adjacentRepetitions->Dict.set(digitKey, {digit, count: 2});
               Match(adjacentRepetitions, digit);
             } else {
               NoMatch(digit);
             }
           | Match(adjacentRepetitions, prevDigit) =>
             if (digit == prevDigit) {
               let digitKey = string_of_int(digit);
               switch (adjacentRepetitions->Dict.get(digitKey)) {
               | None =>
                 adjacentRepetitions->Dict.set(digitKey, {digit, count: 2});
                 Match(adjacentRepetitions, digit);
               | Some({count}) =>
                 adjacentRepetitions->Dict.set(
                   digitKey,
                   {digit, count: count + 1},
                 );
                 Match(adjacentRepetitions, digit);
               };
             } else {
               Match(adjacentRepetitions, digit);
             }
           }
         },
         NoMatch(head),
       );
  adjacentRepetitions;
};

let hasDouble = (withinRepetitionTolerance, password) => {
  let adjacentRepetitions = findAdjacentRepetitions(password);
  switch (adjacentRepetitions) {
  | NoMatch(_) => None
  | Match(adjacentRepetitions, _) =>
    let digitKeys = adjacentRepetitions->Dict.keys;
    digitKeys
    |> Array.fold_left(
         (verdict, digitKey) => {
           switch (verdict) {
           | None =>
             switch (adjacentRepetitions->Dict.get(digitKey)) {
             | None => None
             | Some({count}) =>
               if (count |> withinRepetitionTolerance) {
                 Some(password);
               } else {
                 None;
               }
             }
           | Some(_) => Some(password)
           }
         },
         None,
       );
  };
};

let digitsNeverDecrease = password => {
  let verdict =
    password
    |> Array.fold_left(
         (verdict, digit) => {
           switch (verdict) {
           | None => None
           | Some(prevDigit) =>
             if (digit - prevDigit >= 0) {
               Some(digit);
             } else {
               None;
             }
           }
         },
         Some(0),
       );
  switch (verdict) {
  | None => None
  | Some(_) => Some(password)
  };
};

let hasDoublePart1 = hasDouble(withinRepetitionTolerancePart1);
let hasDoublePart2 = hasDouble(withinRepetitionTolerancePart2);
