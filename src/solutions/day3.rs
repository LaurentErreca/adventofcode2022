use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;


pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let tuples = alphabet.into_iter().enumerate().map(|(idx, letter)| (letter, idx+1)).collect::<Vec<(char, usize)>>();
    let m: HashMap<_, _> = tuples.into_iter().collect();
    let mut sm: usize = 0;

    for st in vecstr {
        let x = st.chars();
        let sz = x.clone().count();
        let y: Vec<Vec<char>> = x
                                  .into_iter()
                                  .chunks(sz/2)
                                  .into_iter()
                                  .map(|c| c.collect())
                                  .collect();

        let compartiment_one = HashSet::<char>::from_iter(y[0].clone());
        let compartiment_two = HashSet::<char>::from_iter(y[1].clone());
        let res = compartiment_one.intersection(&compartiment_two).collect::<Vec<&char>>()[0];
        //println!("{:?}", res);
        sm += m.get(res).unwrap();

    }

    println!("Sum : {:?}", sm);

    return String::from("Wrong value for part");
}