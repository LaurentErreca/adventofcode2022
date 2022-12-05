

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut nb_stacks: usize = 0;
    let mut max_stack_size: usize = 0;
    for st in &vecstr {
        if st.chars().nth(1).unwrap() == '1' {
            let str_stacks = st.replace(" ", "");
            nb_stacks = str_stacks.chars().last().unwrap().to_string().parse::<usize>().unwrap();
            break;
        }
        max_stack_size += 1;
    }
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(max_stack_size); nb_stacks];
    for i in 0..=max_stack_size-1 {
        for j in 0..=nb_stacks-1 {
            if vecstr[i].chars().nth(1+4*j).unwrap() != ' ' { stacks[j].push(vecstr[i].chars().nth(1+4*j).unwrap()); }
        }
    }
    // Reverse stacks
    for k in 0..=nb_stacks-1 {
        stacks[k] = stacks[k].clone().into_iter().rev().collect();
    }
    for li in &vecstr[max_stack_size+2..vecstr.len()] {
        let mut split = li.split(" ");
        let (_, nb_to_move, _, start_stack, _, end_stack) = (split.next().unwrap(),
                                                             split.next().unwrap().parse::<usize>().unwrap(),
                                                             split.next().unwrap(),
                                                             split.next().unwrap().parse::<usize>().unwrap(),
                                                             split.next().unwrap(),
                                                             split.next().unwrap().parse::<usize>().unwrap());
        for _ in 1..=nb_to_move {
            let value_to_push = stacks[start_stack-1].pop().unwrap();
            stacks[end_stack-1].push(value_to_push);
        }
    }
    let mut result: Vec<char> = vec![];
    for r in 0..=nb_stacks-1 {
        result.push(stacks[r][stacks[r].len()-1]);
    }
    return result.into_iter().collect::<String>();
}