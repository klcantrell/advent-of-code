open Day7Helpers;

let part1PhaseConfigurations =
  AmpProgram.part1PhaseSetting |> generateCombinations;

let part1ThrusterSignalComputer = phaseConfigurations => {
  let thrusterSignalResults =
    phaseConfigurations
    |> Array.map(phaseConfig => {
         phaseConfig
         |> Array.fold_left(
              (input, phaseSetting) => {
                module IntCodeComputer =
                  IntCodeComputer.Make({
                    let args = genOfList([phaseSetting, input]);
                    let startingAddress = 0;
                  });
                let executionStatus =
                  IntCodeComputer.processCodes(AmpProgram.codes);
                switch (executionStatus) {
                | Complete({output}) =>
                  switch (output) {
                  | None => raise(InvalidOutput)
                  | Some(result) => result
                  }
                | _ => raise(InvalidOutput)
                };
              },
              AmpProgram.initialInput,
            )
       });
  thrusterSignalResults |> Array.sort((left, right) => right - left);
  thrusterSignalResults;
};

print @@
(part1PhaseConfigurations |> part1ThrusterSignalComputer)->Array.get(0);

let part2PhaseConfigurations =
  AmpProgram.part2PhaseSetting |> generateCombinations;

let rec part2ThrusterSignalComputer =
        (~programState=Dict.make(), ~phaseConfig, ()) => {
  let initializationStatus =
    programState |> initializationStatusOfProgramState;
  switch (initializationStatus) {
  | Idle =>
    let nextProgramState =
      programState
      |> copyProgramState
      |> computeInitialProgramState @@
      phaseConfig;
    part2ThrusterSignalComputer(
      ~programState=nextProgramState,
      ~phaseConfig,
      (),
    );
  | Initialized =>
    let nextProgramState =
      (programState |> copyProgramState)->computeNextProgramState;
    switch (nextProgramState |> executionStatusOfProgramState) {
    | Started
    | InProgress =>
      part2ThrusterSignalComputer(
        ~programState=nextProgramState,
        ~phaseConfig,
        (),
      )
    | Completed => nextProgramState
    };
  | _ => raise(InvalidState)
  };
};

let part2MaxThrusterSignal = phaseConfigurations => {
  let thrusterSignalResults =
    phaseConfigurations
    |> Array.map(phaseConfig =>
         part2ThrusterSignalComputer(~phaseConfig, ())
       )
    |> Array.map(finalProgramState => {
         let finalNodeKey =
           finalProgramState
           |> Dict.keys
           |> Array.to_list
           |> List.rev
           |> List.hd;
         switch (finalProgramState->Dict.get(finalNodeKey)) {
         | None => raise(InvalidState)
         | Some(finalNodeState) =>
           switch (finalNodeState) {
           | Complete({output}) =>
             switch (output) {
             | None => raise(InvalidState)
             | Some(result) => result
             }
           | _ => raise(InvalidState)
           }
         };
       });

  thrusterSignalResults |> Array.sort((left, right) => right - left);
  thrusterSignalResults[0];
};

print @@ part2MaxThrusterSignal(part2PhaseConfigurations);
