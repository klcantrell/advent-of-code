namespace Aoc

open System.IO

module Day1Helpers =
    let caloriesByElf () =
        let rec caloriesByElfAux lines currentGroup parsedData =
            match lines with
            | [] -> currentGroup :: parsedData
            | line :: restLines ->
                if line = "" then
                    let nextParsedData = currentGroup :: parsedData
                    caloriesByElfAux restLines [] nextParsedData
                else
                    let nextGroup = int line :: currentGroup
                    caloriesByElfAux restLines nextGroup parsedData

        let lines = File.ReadLines "day1.txt" |> List.ofSeq
        caloriesByElfAux lines [] [] |> List.map List.sum

module Day1 =
    open Day1Helpers

    let part1 () =
        let max =
            caloriesByElf ()
            |> List.sortDescending
            |> List.take 1
            |> List.head

        printfn $"Day 1, Part 1: {max}"

    let part2 () =
        let top3sum =
            caloriesByElf ()
            |> List.sortDescending
            |> List.take 3
            |> List.sum

        printfn $"Day 1, Part 2: {top3sum}"
