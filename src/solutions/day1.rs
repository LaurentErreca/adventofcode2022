
pub fn solve(part: u8, input: &String) -> String {
    let mut incr: i32 = 0;
    let vecstr: Vec<&str> = input.lines().collect();
    let vecint: Vec<i32> = vecstr.into_iter().map(|x| x.parse().unwrap()).collect();
    let vecint_iter = vecint.iter();
    if part == 1 {
        let mut prev: i32 = -1;
        for i in vecint_iter {
            let iloc: i32 = *i;
            if prev == -1 {
                incr += 0;
            }
            else{
                if prev < iloc {
                    incr += 1;
                }
            }
            prev = iloc;
        }
        return incr.to_string();
    }
    if part == 2 {
        let mut prevsum: i32 = -1;
        let mut win_incr: usize = 0;
        let mut winsum: i32;
        let veclen = vecint.len();
        for i in 0..veclen-2 {
            winsum = vecint[i..i+3].iter().sum::<i32>();
            if prevsum == -1 {
                win_incr += 0;
            }
            else {
                if prevsum < winsum {
                    win_incr += 1;
                }
            }
            prevsum = winsum;
        }
        return win_incr.to_string();
    }
    return String::from("part must be 1 or 2");
}
