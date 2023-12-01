let read_lines ch =
  let rec read_lines_aux acc =
    try
      let line = input_line ch in
      read_lines_aux (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines_aux []

let is_digit x = x >= '0' && x <= '9'

let split_chars str =
  let rec aux i acc = if i < 0 then acc else aux (i - 1) (str.[i] :: acc) in
  aux (String.length str - 1) []

let sum = List.fold_left ( + ) 0

let concat_first_and_last_digit is =
  let aux (fst, _) x =
    let str = String.make 1 x in
    if fst == "" then (str, str) else (fst, str)
  in
  let fst, lst = List.fold_left aux ("", "") is in
  fst ^ lst

let valid_strings =
  [|
    "0";
    "1";
    "2";
    "3";
    "4";
    "5";
    "6";
    "7";
    "8";
    "9";
    "zero";
    "one";
    "two";
    "three";
    "four";
    "five";
    "six";
    "seven";
    "eight";
    "nine";
  |]

let part1 lines =
  print_endline "part1";
  let transform x =
    x |> split_chars |> List.filter is_digit |> concat_first_and_last_digit
    |> int_of_string
  in

  lines |> List.map transform |> sum |> string_of_int |> print_endline

let parse_digit buffer =
  let index =
    Array.find_index (fun y -> String.ends_with ~suffix:y buffer) valid_strings
  in
  Option.map (fun i -> String.get (Array.get valid_strings (i mod 10)) 0) index

let part2 lines =
  print_endline "\npart2";

  let transform x =
    x |> split_chars
    |> List.fold_left
         (fun (buffer, acc) x ->
           let appended = buffer ^ String.make 1 x in

           let values =
             match parse_digit appended with Some c -> c :: acc | None -> acc
           in
           (appended, values))
         ("", [])
    |> fun (_, y) -> concat_first_and_last_digit (List.rev y) |> int_of_string
  in
  lines |> List.map transform |> sum |> string_of_int |> print_endline

let () =
  let lines = read_lines stdin in
  part1 lines;
  part2 lines
