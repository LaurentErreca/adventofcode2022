use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;


pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let tuples = alphabet.into_iter().enumerate().map(|(idx, letter)| (letter, idx+1)).collect::<Vec<(char, usize)>>();
    let m: HashMap<_, _> = tuples.into_iter().collect();
    let mut sm: usize = 0;
    let mut global_nb_line: usize = 0;
    let mut nb_line: usize = 0;
    let mut buffer: Vec<HashSet::<char>> = vec![];

    for st in vecstr {
        let x = st.chars();
        let sz = x.clone().count();
        let y: Vec<Vec<char>> = x.clone()
                                  .into_iter()
                                  .chunks(sz/2)
                                  .into_iter()
                                  .map(|c| c.collect())
                                  .collect();

        let compartiment_one = HashSet::<char>::from_iter(y[0].clone());
        let compartiment_two = HashSet::<char>::from_iter(y[1].clone());
        let res = compartiment_one.intersection(&compartiment_two).collect::<Vec<&char>>()[0];
        if part == 1 { sm += m.get(res).unwrap(); }
        if part == 2 {
            buffer.push(HashSet::<char>::from_iter(x));
            if nb_line == 2 {
                let r = &(&buffer[global_nb_line-2] & &buffer[global_nb_line-1]) & &buffer[global_nb_line];
                let letter = &r.into_iter().collect::<Vec<char>>()[0];
                sm += m.get(letter).unwrap();
                nb_line = 0;
            }
            else { nb_line += 1; }
            global_nb_line += 1;
        }

    }

    if part == 1 || part == 2 { return sm.to_string(); }

    return String::from("Wrong value for part");
}