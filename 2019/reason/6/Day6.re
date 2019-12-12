open Day6Helpers;

let orbitMap = Orbit.data |> createOrbitMap;

let rec countOrbits = (~count=0, ~spaceBall, ~orbitMap, ()) => {
  let nextSpaceBall = orbitMap->Dict.get(spaceBall);
  switch (nextSpaceBall) {
  | None => count
  | Some(next) =>
    countOrbits(~count=count + 1, ~spaceBall=next, ~orbitMap, ())
  };
};

let countOrbitalTransfers = (~from, ~to_, ~orbitMap) => {
  let intersectionPoint = findIntersectingSpaceBall(from, to_, orbitMap);
  let distanceBetweenFromAndIntersection =
    countOrbitalTransfers(~from, ~to_=intersectionPoint, ~orbitMap, ());
  let distanceBetweenToAndIntersection =
    countOrbitalTransfers(~from=to_, ~to_=intersectionPoint, ~orbitMap, ());

  distanceBetweenFromAndIntersection + distanceBetweenToAndIntersection;
};

print(
  orbitMap
  |> Dict.keys
  |> Array.map(spaceBall => countOrbits(~spaceBall, ~orbitMap, ()))
  |> Array.fold_left((+), 0),
);

print(countOrbitalTransfers(~from="YOU", ~to_="SAN", ~orbitMap));
