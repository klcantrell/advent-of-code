let read_lines file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  String.split_on_char '\n' contents

let rec take_aux input_list num_items output_list =
  if num_items = 0 then output_list
  else
    match input_list with
    | [] -> output_list
    | h :: t -> take_aux t (num_items - 1) (h :: output_list)

let take num_items input_list = take_aux input_list num_items []
let sum = List.fold_left ( + ) 0
let sort_descending lst = lst |> List.sort compare |> List.rev
