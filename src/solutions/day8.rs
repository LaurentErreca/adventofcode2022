pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (rem, quot)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x,y)
}

fn print_grid(data: &Vec<u32>, nb_col: usize, nb_row: usize) -> () {
    for nbrow in 0..nb_row { println!("{:?}", &data[nbrow*nb_col..nbrow*nb_col+nb_col]); }
}

fn is_visible(data: &Vec<u32>, index: usize, nb_col: usize, nb_row: usize) -> bool {
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);
    let (x, y) = to_coord(index);
    if x == 0 || x == nb_col-1 || y == 0 || y == nb_row-1 { return true; }

    // top
    let mut max_previous_val = data[to_idx(x, 0)];
    let mut visible = true;
    for yy in 1..=y {
        if yy == y && data[to_idx(x, yy)] <= max_previous_val { visible = false; break; }
        if yy != y && data[to_idx(x, yy)] >= max_previous_val { max_previous_val = data[to_idx(x, yy)]; }
    }
    if visible { return true;}
    visible = true;
    max_previous_val = data[to_idx(x, nb_row-1)];
    // down
    for yy in (y..=nb_row-2).rev() {
        if yy == y && data[to_idx(x, yy)] <= max_previous_val { visible = false; break; }
        if yy != y && data[to_idx(x, yy)] >= max_previous_val { max_previous_val = data[to_idx(x, yy)]; }
    }
    if visible { return true;}
    visible = true;
    max_previous_val = data[to_idx(nb_col-1, y)];
    // right
    for xx in (x..=nb_col-2).rev() {
        if xx == x && data[to_idx(xx, y)] <= max_previous_val { visible = false; break; }
        if xx != x && data[to_idx(xx, y)] >= max_previous_val { max_previous_val = data[to_idx(xx, y)]; }
    }
    if visible { return true;}
    visible = true;
    max_previous_val = data[to_idx(0, y)];
    // left
    for xx in 1..=x {
        if xx == x && data[to_idx(xx, y)] <= max_previous_val { visible = false; break; }
        if xx != x && data[to_idx(xx, y)] >= max_previous_val { max_previous_val = data[to_idx(xx, y)]; }
    }
    if visible { return true;}
    else { return false; }
}

fn compute_scenic_score(data: &Vec<u32>, index: usize, nb_col: usize, nb_row: usize) -> u32 {
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);
    let (x, y) = to_coord(index);

    // top
    let mut value: u32 = data[index];
    let mut direction_score_top: u32 = 0;
    for yy in (0..=y-1).rev() {
        direction_score_top += 1;
        if data[to_idx(x, yy)] >= value {
            break;
        } 
    }

    // down
    value = data[index];
    let mut direction_score_down: u32 = 0;
    for yy in y+1..=nb_row-1 {
        direction_score_down += 1;
        if data[to_idx(x, yy)] >= value {
            break;
        } 
    }

    // left
    value = data[index];
    let mut direction_score_left: u32 = 0;
    for xx in (0..=x-1).rev() {
        direction_score_left += 1;
        if data[to_idx(xx, y)] >= value {
            break;
        } 
    }

    // right
    value = data[index];
    let mut direction_score_right: u32 = 0;
    for xx in x+1..=nb_col-1 {
        direction_score_right += 1;
        if data[to_idx(xx, y)] >= value {
            break;
        } 
    }
    return direction_score_top*direction_score_down*direction_score_left*direction_score_right;
}

pub fn solve(part: u8, input: &String) -> String {
    //let vecstr: Vec<&str> = input.lines().collect();
    let mut nb_col: usize = 0;
    let mut nb_row: usize = 0;
    let mut data: Vec<u32> = vec![];
    for li in input.lines(){
        nb_col = li.len();
        nb_row += 1;
        for c in li.chars() {
            data.push(c as u32 - 0x30);
        }
    }
    println!("Nb col : {}", nb_col);
    println!("Nb row : {}", nb_row);
    let to_coord = |x| div_rem_usize(x, nb_col);
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let mut nb_visible = 0;
    let mut visible_indexes: Vec<usize> = vec![];
    for x in 0..nb_col {
        for y in 0..nb_row {
            if is_visible(&data, to_idx(x, y), nb_col, nb_row) { nb_visible += 1; visible_indexes.push(to_idx(x, y)); }
        }
    }
    if part == 1 { return nb_visible.to_string(); }

    if part == 2 {
        let mut final_score: u32 = 0;
        for idx in visible_indexes {
            let (x, y) = to_coord(idx);
            if x == 0 || x == nb_col-1 || y == 0 || y == nb_row-1 { continue; }
            let score = compute_scenic_score(&data, idx, nb_col, nb_row);
            if score > final_score { final_score = score; }
        }
        return final_score.to_string();
    }


    return nb_visible.to_string();
}
