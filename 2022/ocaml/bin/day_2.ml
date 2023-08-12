type score = { outcome_points : int; shape_points : int }

module Day2Helpers = struct
  let invalid_selection () =
    invalid_arg "Selection must be in one \"X\", \"Y\", or \"Z\""

  let invalid_round () =
    invalid_arg
      "A round should be 3 characters with the second being a space (e.g. A X)"

  let lines_to_scores ~outcome_points_calculator ~shape_points_calculator =
    let rec lines_to_scores_aux lines scores =
      match lines with
      | [] -> scores
      | round :: rest -> (
          if round = "" then lines_to_scores_aux [] scores
          else
            try
              let round_input = String.split_on_char ' ' round in
              match round_input with
              | [ first; second ] ->
                  let new_scores =
                    {
                      outcome_points = outcome_points_calculator first second;
                      shape_points = shape_points_calculator first second;
                    }
                    :: scores
                  in

                  lines_to_scores_aux rest new_scores
              | _ -> invalid_round ()
            with _ -> invalid_round ())
    in

    let lines = Lib.read_lines "bin/day_2.txt" in
    lines_to_scores_aux lines []

  let outcome_points_of_round_part1 other selection =
    match other with
    | "A" -> (
        match selection with
        | "X" -> 3
        | "Y" -> 6
        | "Z" -> 0
        | _ -> invalid_selection ())
    | "B" -> (
        match selection with
        | "X" -> 0
        | "Y" -> 3
        | "Z" -> 6
        | _ -> invalid_selection ())
    | "C" -> (
        match selection with
        | "X" -> 6
        | "Y" -> 0
        | "Z" -> 3
        | _ -> invalid_selection ())
    | _ -> invalid_arg "Other must be in one \"A\", \"B\", or \"C\""

  let outcome_points_of_round_part2 _other selection =
    match selection with
    | "X" -> 0
    | "Y" -> 3
    | "Z" -> 6
    | _ -> invalid_selection ()

  let points_of_shape_part1 _other selection =
    match selection with
    | "X" -> 1
    | "Y" -> 2
    | "Z" -> 3
    | _ -> invalid_selection ()

  let points_of_shape_part2 other selection =
    match other with
    | "A" -> (
        match selection with
        | "X" -> 3
        | "Y" -> 1
        | "Z" -> 2
        | _ -> invalid_selection ())
    | "B" -> (
        match selection with
        | "X" -> 1
        | "Y" -> 2
        | "Z" -> 3
        | _ -> invalid_selection ())
    | "C" -> (
        match selection with
        | "X" -> 2
        | "Y" -> 3
        | "Z" -> 1
        | _ -> invalid_selection ())
    | _ -> invalid_arg "Other must be in one \"A\", \"B\", or \"C\""
end

module Day2 = struct
  let part_1 () =
    let open Day2Helpers in
    let total_score =
      let scores =
        lines_to_scores ~outcome_points_calculator:outcome_points_of_round_part1
          ~shape_points_calculator:points_of_shape_part1
      in

      scores
      |> List.fold_left
           (fun total round ->
             total + round.outcome_points + round.shape_points)
           0
    in

    let _ = print_endline @@ "Day 2, Part 1: " ^ string_of_int total_score in
    ()

  let part_2 () =
    let open Day2Helpers in
    let total_score =
      let scores =
        lines_to_scores ~outcome_points_calculator:outcome_points_of_round_part2
          ~shape_points_calculator:points_of_shape_part2
      in

      scores
      |> List.fold_left
           (fun total round ->
             total + round.outcome_points + round.shape_points)
           0
    in

    let _ = print_endline @@ "Day 2, Part 2: " ^ string_of_int total_score in
    ()
end

let () =
  let _ = Day2.part_1 () in
  let _ = Day2.part_2 () in
  ()
