use std::fs;

#[derive(PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn main() {
    let (total_score_based_on_two_moves, total_score_based_on_move_and_outcome) =
        calculate_total_score_from_file();
    println!(
        "Total score based on two moves: {}",
        total_score_based_on_two_moves
    );
    println!(
        "Total score based on move and outcome: {}",
        total_score_based_on_move_and_outcome
    );
}

fn calculate_total_score_from_file() -> (u32, u32) {
    let rock_paper_scissors_data = fs::read_to_string("rock_paper_scissors_data.txt")
        .expect("Something went wrong reading the file");

    let scores_based_on_two_moves: Vec<u32> = rock_paper_scissors_data
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(calculate_score_based_on_two_moves)
        .collect();

    let score_based_on_move_and_outcome: Vec<u32> = rock_paper_scissors_data
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(calculate_score_based_on_move_and_outcome)
        .collect();

    let total_score_based_on_two_moves = scores_based_on_two_moves.iter().fold(0, |acc, m| acc + m);
    let total_score_based_on_move_and_outcome = score_based_on_move_and_outcome
        .iter()
        .fold(0, |acc, m| acc + m);

    (
        total_score_based_on_two_moves,
        total_score_based_on_move_and_outcome,
    )
}

fn calculate_score_based_on_two_moves(input_str: &str) -> u32 {
    // A,B,C = Rock, Paper, Scissors (OPPONENT)
    // X,Y,Z = Rock, Paper, Scissors (YOU)
    let split = input_str.split(" ").collect::<Vec<&str>>();
    let opponents_move = match split[0] {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        _ => panic!("Invalid input"),
    };
    let your_move = match split[1] {
        "X" => RockPaperScissors::Rock,
        "Y" => RockPaperScissors::Paper,
        "Z" => RockPaperScissors::Scissors,
        _ => panic!("Invalid input"),
    };

    let outcome = calculate_rock_paper_scissor_outcome(&your_move, &opponents_move);

    calculate_score(&your_move, &outcome)
}

fn calculate_rock_paper_scissor_outcome(
    your_move: &RockPaperScissors,
    opponents_move: &RockPaperScissors,
) -> Outcome {
    if (your_move == &RockPaperScissors::Rock && opponents_move == &RockPaperScissors::Scissors)
        || (your_move == &RockPaperScissors::Paper && opponents_move == &RockPaperScissors::Rock)
        || (your_move == &RockPaperScissors::Scissors
            && opponents_move == &RockPaperScissors::Paper)
    {
        Outcome::Win
    } else if your_move == opponents_move {
        Outcome::Draw
    } else {
        Outcome::Lose
    }
}

fn calculate_score_based_on_move_and_outcome(input_str: &str) -> u32 {
    // A,B,C = Rock, Paper, Scissors (OPPONENT)
    // X,Y,Z = Lose, Draw, Win (OUTCOME)
    let split = input_str.split(" ").collect::<Vec<&str>>();
    let opponents_move = match split[0] {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        _ => panic!("Invalid input"),
    };
    let outcome = match split[1] {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid input"),
    };

    let your_move = calculate_your_move(&opponents_move, &outcome);

    calculate_score(&your_move, &outcome)
}

fn calculate_your_move(opponents_move: &RockPaperScissors, outcome: &Outcome) -> RockPaperScissors {
    match outcome {
        Outcome::Win => match opponents_move {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        },
        Outcome::Lose => match opponents_move {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        },
        Outcome::Draw => match opponents_move {
            RockPaperScissors::Rock => RockPaperScissors::Rock,
            RockPaperScissors::Paper => RockPaperScissors::Paper,
            RockPaperScissors::Scissors => RockPaperScissors::Scissors,
        },
    }
}

fn calculate_score(you: &RockPaperScissors, outcome: &Outcome) -> u32 {
    let scissor_bonus = 3;
    let paper_bonus = 2;
    let rock_bonus = 1;

    let win_bonus = 6;
    let draw_bonus = 3;
    let lose_bonus = 0;

    match outcome {
        Outcome::Win => match you {
            RockPaperScissors::Rock => win_bonus + rock_bonus,
            RockPaperScissors::Paper => win_bonus + paper_bonus,
            RockPaperScissors::Scissors => win_bonus + scissor_bonus,
        },
        Outcome::Lose => match you {
            RockPaperScissors::Rock => lose_bonus + rock_bonus,
            RockPaperScissors::Paper => lose_bonus + paper_bonus,
            RockPaperScissors::Scissors => lose_bonus + scissor_bonus,
        },
        Outcome::Draw => match you {
            RockPaperScissors::Rock => draw_bonus + rock_bonus,
            RockPaperScissors::Paper => draw_bonus + paper_bonus,
            RockPaperScissors::Scissors => draw_bonus + scissor_bonus,
        },
    }
}

#[test]
fn test_day_2() {
    let (total_score_based_on_two_moves, total_score_based_on_move_and_outcome) =
        calculate_total_score_from_file();
    assert_eq!(total_score_based_on_two_moves, 11906);
    assert_eq!(total_score_based_on_move_and_outcome, 11186);
}
