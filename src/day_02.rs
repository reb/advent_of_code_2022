/// --- Day 2: Rock Paper Scissors ---
///
/// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the
/// snack storage, a giant Rock Paper Scissors tournament is already in progress.
///
/// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each
/// round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand
/// shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper,
/// and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
///
/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
/// input) that they say will be sure to help you win. "The first column is what your opponent is
/// going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the
/// Elf is called away to help with someone's tent.
///
/// The second column, you reason, must be what you should play in response: X for Rock, Y for
/// Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have
/// been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score. Your total score is the
/// sum of your scores for each round. The score for a single round is the score for the shape you
/// selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the
/// round (0 if you lost, 3 if the round was a draw, and 6 if you won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
/// score you would get if you were to follow the strategy guide.
///
/// For example, suppose you were given the following strategy guide:
///
/// A Y
/// B X
/// C Z
///
/// This strategy guide predicts and recommends the following:
///
///     In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
///     This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you
///     won).
///     In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
///     This ends in a loss for you with a score of 1 (1 + 0).
///     The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 =
///     6.
///
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8
/// + 1 + 6).
///
/// What would your total score be if everything goes exactly according to your strategy guide?
///
/// --- Part Two ---
///
/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column
/// says how the round needs to end: X means you need to lose, Y means you need to end the round in
/// a draw, and Z means you need to win. Good luck!"
///
/// The total score is still calculated in the same way, but now you need to figure out what shape
/// to choose so the round ends as indicated. The example above now goes like this:
///
///
///    In the first round, your opponent will choose Rock (A), and you need the round to end in a
///    draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
///    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X)
///    with a score of 1 + 0 = 1.
///    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 =
///    7.
///
/// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total
/// score of 12.
///
/// Following the Elf's instructions for the second column, what would your total score be if
/// everything goes exactly according to your strategy guide?
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_02");

pub fn run() {
    let guide = load_guide(INPUT);

    let strategy = translate_guide(&guide);

    let score = score_strategy(&strategy);
    println!(
        "The total score according to the strategy guide is: {}",
        score
    );
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

fn translate_guide(guide: &Vec<(char, char)>) -> Vec<(Sign, Sign)> {
    let mut key = HashMap::new();
    key.insert('A', Sign::Rock);
    key.insert('B', Sign::Paper);
    key.insert('C', Sign::Scissors);
    key.insert('X', Sign::Rock);
    key.insert('Y', Sign::Paper);
    key.insert('Z', Sign::Scissors);
    guide
        .iter()
        .map(|(opponent, own)| (*key.get(opponent).unwrap(), *key.get(own).unwrap()))
        .collect()
}

fn load_guide(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|character| {
                    character
                        .chars()
                        .next()
                        .expect("There should be a character")
                })
                .collect_tuple()
        })
        .flatten()
        .collect()
}

fn score_strategy(strategy: &Vec<(Sign, Sign)>) -> u32 {
    strategy.iter().map(round_score).sum()
}

fn round_score((opponent_sign, own_sign): &(Sign, Sign)) -> u32 {
    // first calculate score for the own sign
    let mut score = match own_sign {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissors => 3,
    };

    // if it's draw add 3
    if opponent_sign == own_sign {
        score += 3;
    }

    // if it's a victory add 6
    if opponent_sign == &Sign::Rock && own_sign == &Sign::Paper {
        score += 6;
    }
    if opponent_sign == &Sign::Paper && own_sign == &Sign::Scissors {
        score += 6;
    }
    if opponent_sign == &Sign::Scissors && own_sign == &Sign::Rock {
        score += 6;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_guide() {
        let input = "A Y\nB X\nC Z";

        let expected = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];

        assert_eq!(load_guide(input), expected);
    }
    #[test]
    fn test_score_strategy() {
        // For example, suppose you were given the following strategy guide:
        //
        // A Y
        // B X
        // C Z
        //
        // In this example, if you were to follow the strategy guide, you would get a total score of 15
        // (8 + 1 + 6).
        let strategy = vec![
            (Sign::Rock, Sign::Paper),
            (Sign::Paper, Sign::Rock),
            (Sign::Scissors, Sign::Scissors),
        ];

        assert_eq!(score_strategy(&strategy), 15);
    }

    #[test]
    fn test_round_score() {
        // In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
        // This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you
        // won).
        assert_eq!(round_score(&(Sign::Rock, Sign::Paper)), 8);
        // In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
        // This ends in a loss for you with a score of 1 (1 + 0).
        assert_eq!(round_score(&(Sign::Paper, Sign::Rock)), 1);
        // The third round is a draw with both players choosing Scissors, giving you a score of
        // 3 + 3 = 6.
        assert_eq!(round_score(&(Sign::Scissors, Sign::Scissors)), 6);
    }
}
