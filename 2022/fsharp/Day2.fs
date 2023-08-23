namespace Aoc

open System.IO

module Day2 =
    type Score =
        { outcomePoints: int
          shapePoints: int }

    module private Helpers =
        let invalidRound () =
            invalidArg "roundInput" "A round should be 3 characters with the second being a space (e.g. A X)"

        let invalidSelection () =
            invalidArg "selection" "Selection must be in one \"X\", \"Y\", or \"Z\""

        let invalidOpponentSelection () =
            invalidArg "opponentSelection" "Opponent selection must be in one \"X\", \"Y\", or \"Z\""

        [<AbstractClass; Sealed>]
        type Utils =
            static member linesToScores(outcomePointsCalculator, shapePointsCalculator) =
                let rec linesToScoresAux lines scores =
                    match lines with
                    | [] -> scores
                    | round :: rest ->
                        if round = "" then
                            linesToScoresAux [] scores
                        else
                            let roundInput = round.Split ' '

                            match roundInput with
                            | [| first; second |] ->
                                let newScores =
                                    { outcomePoints = outcomePointsCalculator first second
                                      shapePoints = shapePointsCalculator first second }
                                    :: scores

                                linesToScoresAux rest newScores
                            | _ -> invalidRound ()

                let lines = File.ReadLines "day2.txt" |> List.ofSeq
                linesToScoresAux lines []

            static member outcomePointsOfRoundPart1 other selection =
                match other with
                | "A" ->
                    (match selection with
                     | "X" -> 3
                     | "Y" -> 6
                     | "Z" -> 0
                     | _ -> invalidSelection ())
                | "B" ->
                    (match selection with
                     | "X" -> 0
                     | "Y" -> 3
                     | "Z" -> 6
                     | _ -> invalidSelection ())
                | "C" ->
                    (match selection with
                     | "X" -> 6
                     | "Y" -> 0
                     | "Z" -> 3
                     | _ -> invalidSelection ())
                | _ -> invalidOpponentSelection ()

            static member pointsOfShapePart1 _other selection =
                match selection with
                | "X" -> 1
                | "Y" -> 2
                | "Z" -> 3
                | _ -> invalidSelection ()

    open type Helpers.Utils

    let part1 () =
        let totalScore =
            linesToScores (
                outcomePointsCalculator = outcomePointsOfRoundPart1,
                shapePointsCalculator = pointsOfShapePart1
            )
            |> List.fold (fun total round -> total + round.outcomePoints + round.shapePoints) 0

        printfn $"Day 2, Part 1 {totalScore}"

    let part2 () = printfn "Day 2, Part 2"
