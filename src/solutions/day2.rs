use std::collections::HashMap;

fn get_score(opponent_game: &str, your_game: &str) -> i32 {
    if opponent_game == "Rock" && your_game == "Paper" {
        return 8;
    }
    if opponent_game == "Rock" && your_game == "Scissors" {
        return 3;
    }
    if opponent_game == "Rock" && your_game == "Rock" {
        return 4;
    }
    if opponent_game == "Paper" && your_game == "Paper" {
        return 5;
    }
    if opponent_game == "Paper" && your_game == "Rock" {
        return 1;
    }
    if opponent_game == "Paper" && your_game == "Scissors" {
        return 9;
    }
    if opponent_game == "Scissors" && your_game == "Scissors" {
        return 6;
    }
    if opponent_game == "Scissors" && your_game == "Rock" {
        return 7;
    }
    if opponent_game == "Scissors" && your_game == "Paper" {
        return 2;
    }
    return 0;
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut total_scores: i32 = 0;
    let mut LetterToChoice: HashMap<&str, &str> = HashMap::from([
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
        total_scores += get_score(LetterToChoice.get(opponent_game).unwrap(), LetterToChoice.get(your_game).unwrap());
    }
    if part == 1 {
        return total_scores.to_string();
    }
/*     else if part == 2 {
        all_results.sort();
        let res: i32 = all_results.iter().rev().take(3).sum();
        return res.to_string();
    } */
    return String::from("Wrong value for part");
}
