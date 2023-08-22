namespace Aoc

type Score = { outcomePoints: int; shapePoint: int }

[<AbstractClass; Sealed>]
type Day2Helpers =
    static member todo(message: string) = message

module Day2 =
    open type Day2Helpers

    let part1 () =
        let message = todo (message = "hello")
        printfn $"Day 2, Part 1 {message}"

    let part2 () = printfn "Day 2, Part 2"
