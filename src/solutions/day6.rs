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
    let mut buffer: Vec<char> = vec![];
    for c in data.clone() {
        buffer.push(c);
        nb_chars += 1;
        if nb_chars >= length_message_identifier {
            let hs: usize = HashSet::<char>::from_iter(buffer[buffer.len()-length_message_identifier..].to_vec()).len();
            if hs == length_message_identifier {
                return nb_chars.to_string();
            }
        }
    }
    return String::from("Wrong value for part");
}