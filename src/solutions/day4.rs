use std::collections::HashSet;


pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut result: usize = 0;
    for st in vecstr {
        let mut split = st.split(",");
        let range_elve1 = split.next().unwrap();
        let range_elve2 = split.next().unwrap();
        let mut split_elve1 = range_elve1.split("-");
        let mut split_elve2 = range_elve2.split("-");
        let v_range_elve1: Vec<i32>  = (split_elve1.next().unwrap().parse::<i32>().unwrap()..split_elve1.next().unwrap().parse::<i32>().unwrap()+1).collect();
        let v_range_elve2: Vec<i32>  = (split_elve2.next().unwrap().parse::<i32>().unwrap()..split_elve2.next().unwrap().parse::<i32>().unwrap()+1).collect();

        let h_range1 = HashSet::<i32>::from_iter(v_range_elve1);
        let h_range2 = HashSet::<i32>::from_iter(v_range_elve2);
        let intersect =  &h_range1 & &h_range2;
        if part == 1 { if &intersect.len() >= &h_range1.len() || &intersect.len() >= &h_range2.len() { result += 1; } }
        if part == 2 { if &intersect.len() > &(0 as usize) { result += 1 } }
    }
    if part == 1 || part == 2 { return result.to_string(); }
    return String::from("Wrong value for part");
}