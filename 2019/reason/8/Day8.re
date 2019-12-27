open Day8Helpers;

let part1PasswordVerification =
  Password.value
  |> partition(SpaceImageFormat.layerArea)
  |> List.map(countDigits)
  |> List.sort(byZeroCountAscending)
  |> List.hd
  |> (
    counter => {
      let oneCount =
        switch (counter->Dict.get("1")) {
        | None => 0
        | Some(c) => c
        };
      let twoCount =
        switch (counter->Dict.get("2")) {
        | None => 0
        | Some(c) => c
        };
      oneCount * twoCount;
    }
  );

print(part1PasswordVerification);

Password.value
|> partition(SpaceImageFormat.layerArea)
|> List.rev
|> List.fold_left(
     (message, layer) => {
       let charactersOfMessage = message |> split("");
       charactersOfMessage
       |> Array.iteri((i, c) => {
            let swap =
              switch (c |> pixelColorOfCharacter) {
              | Black
              | White => None
              | Transparent => Some(layer |> charAt(i))
              };
            switch (swap) {
            | None => ()
            | Some(value) => charactersOfMessage[i] = value
            };
          });
       charactersOfMessage |> join("");
     },
     Transparent
     |> characterOfPixelColor
     |> repeat(SpaceImageFormat.layerArea),
   )
|> renderImage;
