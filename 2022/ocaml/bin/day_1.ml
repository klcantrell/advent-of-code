let calories_by_elf () = Lib.read_lines "bin/day_1.txt" |> List.iter print_endline

module Day1 = struct
  let part_1 () =
    let _ = print_endline "Day 1, Part 1" in
    let _ = calories_by_elf () in
    ()
  let part_2 () = print_endline "Day 1, Part 2"
end

let () =
  let _ = Day1.part_1 () in
  let _ = Day1.part_2 () in
  ()
