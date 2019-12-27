exception InvalidPasswordCharacter;

type pixelColor =
  | Black
  | White
  | Transparent;

let pixelColorOfCharacter =
  fun
  | "0" => Black
  | "1" => White
  | "2" => Transparent
  | _ => raise(InvalidPasswordCharacter);

let characterOfPixelColor =
  fun
  | Black => "0"
  | White => "1"
  | Transparent => "2";

let print = Js.log;
let split = Js.String.split;
let slice = Js.String.slice;
let length = Js.String.length;
let charAt = Js.String.charAt;
let repeat = Js.String.repeat;
let join = Js.Array.joinWith;

module Dict = {
  type t('a) = Js.Dict.t('a);
  let make = Js.Dict.empty;
  let get = Js.Dict.get;
  let set = Js.Dict.set;
  let keys = Js.Dict.keys;
  let entries = Js.Dict.entries;
  let fromArray = Js.Dict.fromArray;
};

module SpaceImageFormat = {
  let layerWidth = 25;
  let layerHeight = 6;
  let layerArea = layerWidth * layerHeight;
};

let partition = (size, str) => {
  let rec traverse = (~layers=[], ~str, ()) => {
    switch (str) {
    | "" => layers
    | _ =>
      let layer = str |> slice(~from=0, ~to_=size);
      let rest = str |> slice(~from=size, ~to_=length(str));
      traverse(~layers=[layer, ...layers], ~str=rest, ());
    };
  };
  traverse(~str, ());
};

let countDigits = str => {
  let rec count = (~counter=Dict.make(), ~str, ()) => {
    switch (str) {
    | "" => counter
    | s =>
      let digit = s |> slice(~from=0, ~to_=1);
      let rest = s |> slice(~from=1, ~to_=length(s));
      switch (counter->Dict.get(digit)) {
      | None => counter->Dict.set(digit, 1)
      | Some(c) => counter->Dict.set(digit, c + 1)
      };
      count(~counter, ~str=rest, ());
    };
  };
  count(~str, ());
};

let byZeroCountAscending = (left, right) => {
  let leftZeroCount =
    switch (left->Dict.get("0")) {
    | None => 0
    | Some(c) => c
    };
  let rightZeroCount =
    switch (right->Dict.get("0")) {
    | None => 0
    | Some(c) => c
    };
  leftZeroCount - rightZeroCount;
};

let renderImage = pixelStream =>
  pixelStream
  |> split("")
  |> Array.map(
       fun
       | "0" => " "
       | _ => "*",
     )
  |> join("")
  |> partition(SpaceImageFormat.layerWidth)
  |> List.rev
  |> List.iter(print);
