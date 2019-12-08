open Day4Helpers;

let satisfyRequirementsPart1 = password => {
  let verdict = password |> hasDoublePart1 >>= digitsNeverDecrease;
  switch (verdict) {
  | None => YouShallNotPass
  | Some(_) => GoodToGo
  };
};

let satisfyRequirementsPart2 = password => {
  let verdict = password |> hasDoublePart2 >>= digitsNeverDecrease;
  switch (verdict) {
  | None => YouShallNotPass
  | Some(_) => GoodToGo
  };
};

let numberOfPasswordsThatSatisfyRequirements =
    (rangeStart, rangeEnd, requirements) => {
  let rec traverse = (~count=0, ~password, ()) =>
    if (password > rangeEnd) {
      count;
    } else {
      let passwordDigits = digitsOfPassword(password);
      let nextPassword = password + 1;
      switch (passwordDigits |> requirements) {
      | YouShallNotPass => traverse(~count, ~password=nextPassword, ())
      | GoodToGo => traverse(~count=count + 1, ~password=nextPassword, ())
      };
    };
  traverse(~password=rangeStart, ());
};

let passwordsThat =
  numberOfPasswordsThatSatisfyRequirements(
    PasswordInput.rangeStart,
    PasswordInput.rangeEnd,
  );

print(passwordsThat(satisfyRequirementsPart1));

print(passwordsThat(satisfyRequirementsPart2));
