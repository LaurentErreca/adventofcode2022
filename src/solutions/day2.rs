use std::collections::HashMap;

fn get_score(opponent_game: &str, your_game: &str) -> i32 {
    if opponent_game == "Rock" && your_game == "Paper" { return 8; }
    if opponent_game == "Rock" && your_game == "Scissors" { return 3; }
    if opponent_game == "Rock" && your_game == "Rock" { return 4; }
    if opponent_game == "Paper" && your_game == "Paper" { return 5; }
    if opponent_game == "Paper" && your_game == "Rock" { return 1; }
    if opponent_game == "Paper" && your_game == "Scissors" { return 9; }
    if opponent_game == "Scissors" && your_game == "Scissors" { return 6; }
    if opponent_game == "Scissors" && your_game == "Rock" { return 7; }
    if opponent_game == "Scissors" && your_game == "Paper" { return 2; }
    return 0;
}

fn get_expected<'a>(opponent: &'a str, result: &'a str) -> &'a str {
    if opponent == "Rock" && result == "Z" { return "Paper"; }
    if opponent == "Rock" && result == "Y" { return "Rock"; }
    if opponent == "Rock" && result == "X" { return "Scissors"; }
    if opponent == "Paper" && result == "Z" { return "Scissors"; }
    if opponent == "Paper" && result == "Y" { return "Paper"; }
    if opponent == "Paper" && result == "X" { return "Rock"; }
    if opponent == "Scissors" && result == "Z" { return "Rock"; }
    if opponent == "Scissors" && result == "Y" { return "Scissors"; }
    if opponent == "Scissors" && result == "X" { return "Paper"; }
    return "Error";
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut total_scores: i32 = 0;
    let letter_to_choice: HashMap<&str, &str> = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors"),
        ("X", "Rock"),
        ("Y", "Paper"),
        ("Z", "Scissors"),
    ]);
    for st in vecstr {
        let mut game = st.split(" ");
        let (opponent_game, your_game) = (game.next().unwrap(), game.next().unwrap());
        if part == 1 {
            total_scores += get_score(letter_to_choice.get(opponent_game).unwrap(), letter_to_choice.get(your_game).unwrap());
        }
        else if part == 2 {
            total_scores += get_score(letter_to_choice.get(opponent_game).unwrap(), get_expected(&letter_to_choice.get(opponent_game).unwrap(), &your_game));
        }
        
    }
    if part == 1 || part == 2 {
        return total_scores.to_string();
    }
    return String::from("Wrong value for part");
}
