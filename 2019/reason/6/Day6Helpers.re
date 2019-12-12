exception NoSpaceBallFound;

module Dict = {
  type t('a) = Js.Dict.t('a);
  let make = Js.Dict.empty;
  let get = Js.Dict.get;
  let set = Js.Dict.set;
  let keys = Js.Dict.keys;
};

let split = Js.String.split;

let print = Js.log;

let rec countOrbitalTransfers = (~count=0, ~from, ~orbitMap, ~to_, ()) => {
  let nextSpaceBall = orbitMap->Dict.get(from);
  switch (nextSpaceBall) {
  | None => count
  | Some(next) =>
    if (next == to_) {
      count;
    } else {
      countOrbitalTransfers(
        ~count=count + 1,
        ~from=next,
        ~to_,
        ~orbitMap,
        (),
      );
    }
  };
};

let findIntersectingSpaceBall =
    (initialFirstSpaceBall, initialSecondSpaceBall, orbitMap) => {
  let rec traverse = (firstSpaceBall, secondSpaceBall) => {
    let firstSpaceBallOrbitee = orbitMap->Dict.get(firstSpaceBall);
    let secondSpaceBallOrbitee = orbitMap->Dict.get(secondSpaceBall);
    switch (firstSpaceBallOrbitee, secondSpaceBallOrbitee) {
    | (None, _) => raise(NoSpaceBallFound)
    | (Some(nextFirstSpaceBall), None) =>
      traverse(nextFirstSpaceBall, initialSecondSpaceBall)
    | (Some(nextFirstSpaceBall), Some(nextSecondSpaceBall)) =>
      if (nextFirstSpaceBall == nextSecondSpaceBall) {
        nextFirstSpaceBall;
      } else {
        traverse(firstSpaceBall, nextSecondSpaceBall);
      }
    };
  };
  traverse(initialFirstSpaceBall, initialSecondSpaceBall);
};

let createOrbitMap = orbitData => {
  orbitData
  |> Array.map(orbitDatum => orbitDatum |> split(")"))
  |> Array.fold_left(
       (orbitsMap, orbitPair) => {
         orbitsMap->Dict.set(orbitPair[1], orbitPair[0]);
         orbitsMap;
       },
       Dict.make(),
     );
};
