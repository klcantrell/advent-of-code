let sum = List.fold_left((+), 0);
let fuelOfMass = mass => mass / 3 - 2;

let calculateFuelPart1 = moduleMasses =>
  moduleMasses |> List.map(fuelOfMass) |> sum;

let calculateFuelPart2 = moduleMasses => {
  let rec calculateTotalFuel = mass =>
    switch (fuelOfMass(mass)) {
    | fuel when fuel <= 0 => 0
    | fuel => fuel + calculateTotalFuel(fuel)
    };
  moduleMasses |> List.map(calculateTotalFuel) |> sum;
};
