let read_lines ch =
  let rec read_lines_aux acc =
    try
      let line = input_line ch in
      read_lines_aux (Seq.cons line acc)
    with End_of_file -> acc
  in
  read_lines_aux Seq.empty

let is_digit x = x >= '0' && x <= '9'

let split_chars str =
  let rec aux i acc =
    if i < 0 then acc else aux (i - 1) (Seq.cons str.[i] acc)
  in
  aux (String.length str - 1) Seq.empty

let sum = Seq.fold_left ( + ) 0

let concat_first_and_last_digit is =
  let aux (fst, _) x =
    let str = String.make 1 x in
    if fst == "" then (str, str) else (fst, str)
  in
  let fst, lst = Seq.fold_left aux ("", "") is in
  fst ^ lst

let part1 lines =
  print_endline "part1";
  let line_to_number x =
    x |> split_chars |> Seq.filter is_digit |> concat_first_and_last_digit
    |> int_of_string
  in

  lines |> Seq.map line_to_number |> sum |> string_of_int |> print_endline

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

let parse_digit buffer =
  let index =
    Array.find_index (fun y -> String.ends_with ~suffix:y buffer) valid_strings
  in
  Option.map (fun i -> String.get (Array.get valid_strings (i mod 10)) 0) index

let append_string str c = str ^ String.make 1 c

let part2 lines =
  print_endline "\npart2";

  let line_to_number x =
    x |> split_chars |> Seq.scan append_string "" |> Seq.filter_map parse_digit
    |> concat_first_and_last_digit |> int_of_string
  in
  lines |> Seq.map line_to_number |> sum |> string_of_int |> print_endline

let () =
  let lines = read_lines stdin in
  part1 lines;
  part2 lines
