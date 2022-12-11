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

fn is_visible(data: Vec<u32>, index: usize, nb_col: usize, nb_row: usize, from_where: &String) -> bool {
    let value = data[index];
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);

    let (x, y) = to_coord(index);
    if x == 0 or x == nb_col-1 or y == 0 or y == nb_row-1 { return true; }

    // top
    for yy in 0..y {
        if data[to_idx(x, yy)] >= value { return false; }
    }
    // down
    for yy in nb_row-1..y {
        if data[to_idx(x, yy)] >= value { return false; }
    }
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

    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);

    for x in 0..nb_col {
        for y in 0..nb_row {

        }
    }
    return String::from("day 8");
}