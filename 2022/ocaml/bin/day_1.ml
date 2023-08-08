module Day1Helpers = struct
  let rec calories_by_elf_aux lines current_group parsed_data =
    match lines with
    | [] -> current_group :: parsed_data
    | line :: rest_lines ->
        if line = "" then
          let next_parsed_data = current_group :: parsed_data in
          calories_by_elf_aux rest_lines [] next_parsed_data
        else
          let next_group = int_of_string line :: current_group in
          calories_by_elf_aux rest_lines next_group parsed_data

  let calories_by_elf () =
    let lines = Lib.read_lines "bin/day_1.txt" in
    calories_by_elf_aux lines [] [] |> List.map Lib.sum
end

module Day1 = struct
  open Day1Helpers

  let part_1 () =
    let max =
      calories_by_elf () |> Lib.sort_descending |> Lib.take 1 |> Lib.sum
    in
    let _ = print_endline @@ "Day 1, Part 1: " ^ string_of_int max in
    ()

  let part_2 () =
    let top_three_sum =
      calories_by_elf () |> Lib.sort_descending |> Lib.take 3 |> Lib.sum
    in
    let _ = print_endline @@ "Day 1, Part 2: " ^ string_of_int top_three_sum in
    ()
end

let () =
  let _ = Day1.part_1 () in
  let _ = Day1.part_2 () in
  ()
