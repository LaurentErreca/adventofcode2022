use std::collections::HashSet;

pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (rem, quot)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x,y)
}

fn compute_position(
    nb_col: usize,
    head_initial_position: usize,
    tail_initial_position: usize,
    direction: &str,
    nb_moves: usize
) -> ((usize, usize), (usize, usize), Vec<usize>) {
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);
    let (mut xh, mut yh) = to_coord(head_initial_position);
    let (mut xt, mut yt) = to_coord(tail_initial_position);
    let mut visited_positions: Vec<usize> = vec![];
    for _ in 0..nb_moves {
        // update head position
        match direction {
            "R" => xh += 1,
            "L" => xh -= 1,
            "U" => yh -= 1,
            "D" => yh += 1,
            _   => println!("Parsing error"),
        }

        // Update tail position
        if ((xh as i32 - xt as i32) as i32).abs() > 1 || ((yh as i32 - yt as i32) as i32).abs() > 1 {
            if xh == xt && yh < yt { yt -= 1; }
            if xh == xt && yh > yt { yt += 1; }
            if yh == yt && xh < xt { xt -= 1; }
            if yh == yt && xh > xt { xt += 1; }
            if xh > xt && yh > yt { xt += 1; yt += 1; }
            if xh > xt && yh < yt { xt += 1;  yt -= 1; }
            if xh < xt && yh > yt { xt -= 1; yt += 1; }
            if xh < xt && yh < yt { xt -= 1; yt -= 1; }
            visited_positions.push(to_idx(xt, yt));
        }
    }
    ((xh, yh), (xt, yt), visited_positions)
}

pub fn solve(part: u8, input: &String) -> String {
    let nb_col: usize = 1000;
    let to_idx = |x: usize, y: usize| y*nb_col + x;

    // Initial position
    let (mut x_head, mut y_head) = (500, 500);
    let (mut x_tail, mut y_tail) = (500, 500);
    let mut visited_pos: Vec<usize> = vec![];
    visited_pos.push(to_idx(x_tail, y_tail));
    for li in input.lines() {
        let mut spl = li.split(" ");
        let (direction, nb) = (spl.next().unwrap(), spl.next().unwrap().parse::<usize>().unwrap());
        let tail_positions: Vec<usize>;

        ((x_head, y_head), (x_tail, y_tail), tail_positions) = compute_position(
            nb_col,
            to_idx(x_head, y_head),
            to_idx(x_tail, y_tail),
            direction,
            nb
        );

        visited_pos.extend(tail_positions);
    }
    if part == 1 {
        return HashSet::<usize>::from_iter(visited_pos).len().to_string();
    }
    return String::from("Wrong value for part");
}