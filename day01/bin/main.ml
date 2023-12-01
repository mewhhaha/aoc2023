let read_lines ch =
  let rec read_lines_aux acc =
    try
      let line = input_line ch in
      read_lines_aux (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines_aux []

let parse_digit x = if x >= '0' && x <= '9' then Some x else None

let split_chars str =
  let rec aux i acc = if i < 0 then acc else aux (i - 1) (str.[i] :: acc) in
  aux (String.length str - 1) []

let concat_chars a b = Printf.sprintf "%c%c" a b

let concat_first_and_last_digit is =
  let aux (fst, _) x =
    let str = String.make 1 x in
    if fst == "" then (str, str) else (fst, str)
  in
  let fst, lst = List.fold_left aux ("", "") is in
  fst ^ lst

let () =
  let transform x =
    x |> split_chars
    |> List.filter_map parse_digit
    |> concat_first_and_last_digit |> int_of_string
  in

  read_lines stdin |> List.map transform |> List.fold_left ( + ) 0
  |> string_of_int |> print_endline
