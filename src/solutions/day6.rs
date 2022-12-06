use std::collections::HashSet;

pub fn solve(part: u8, input: &String) -> String {
    let data: std::str::Chars = input.lines().into_iter().next().unwrap().chars();
    let mut nb_chars: usize = 0;
    let mut length_message_identifier: usize = 0;
    match part {
        1 => length_message_identifier = 4,
        2 => length_message_identifier = 14,
        _ => println!("Incorrect value for part"),
    }
    for _ in data.clone() {
        nb_chars += 1;
        if nb_chars >= length_message_identifier {
            let hs: usize = HashSet::<char>::from_iter(data.clone()
                                                       .into_iter()
                                                       .enumerate()
                                                       .filter(|(idx, _)| idx >= &(nb_chars-length_message_identifier) && idx < &nb_chars )
                                                       .map(|(_, val)| val)
                                                       .collect::<Vec<char>>()
                                                    ).len();
            if hs == length_message_identifier {
                return nb_chars.to_string();
            }
        }
    }
    return String::from("Wrong value for part");
}