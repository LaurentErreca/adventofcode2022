
pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut max_cal: i32 = 0;
    let mut current_elf_cal: i32 = 0;
    let mut all_results: Vec<i32> = vec![];
    for st in vecstr {
        if st != "" {
            current_elf_cal += st.parse::<i32>().unwrap();
        }
        else {
            if current_elf_cal > max_cal {
                max_cal = current_elf_cal;   
            }
            all_results.push(current_elf_cal);
            current_elf_cal = 0;
        }

    }
    if part == 1 {
        return max_cal.to_string();
    }
    else if part == 2 {
        all_results.sort();
        let res: i32 = all_results.iter().rev().take(3).sum();
        return res.to_string();
    }
    return String::from("Wrong value for part");
}
