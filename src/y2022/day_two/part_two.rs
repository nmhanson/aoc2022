use rps::*;
use std::str::FromStr;

pub fn part_two(input: &str) {
    let total: u32 = input
        .trim()
        .split('\n')
        .map(|rnd_str| RPSRound::from_str(rnd_str).unwrap())
        .map(|rnd| rnd.score())
        .sum();
    println!("part_two: {}", total)
}

mod rps {
    use std::str::FromStr;

    pub struct RPSRound {
        them: Move,
        outcome: Outcome,
    }

    #[derive(Debug)]
    enum Move {
        ROCK,
        PAPER,
        SCISSORS,
    }

    #[derive(Debug)]
    enum Outcome {
        LOSS,
        DRAW,
        WIN,
    }

    impl FromStr for RPSRound {
        type Err = ParseRoundError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut tokens = s.split(' ');
            let opponent = tokens.next();
            let outcome = tokens.next();
            match (opponent, outcome) {
                (Some(opp), Some(out)) => Ok(RPSRound {
                    outcome: Outcome::from_str(out)?,
                    them: Move::from_str(opp)?,
                }),
                _ => Err(ParseRoundError("token was none")),
            }
        }
    }

    impl FromStr for Move {
        type Err = ParseRoundError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" | "X" => Ok(Move::ROCK),
                "B" | "Y" => Ok(Move::PAPER),
                "C" | "Z" => Ok(Move::SCISSORS),
                _ => Err(ParseRoundError("failed to parse move")),
            }
        }
    }

    impl FromStr for Outcome {
        type Err = ParseRoundError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Outcome::LOSS),
                "Y" => Ok(Outcome::DRAW),
                "Z" => Ok(Outcome::WIN),
                _ => Err(ParseRoundError("failed to parse outcome")),
            }
        }
    }

    #[derive(Debug)]
    pub struct ParseRoundError(&'static str);

    impl RPSRound {
        pub fn score(&self) -> u32 {
            let my_move: Move = match (&self.them, &self.outcome) {
                (Move::ROCK, Outcome::LOSS) => Move::SCISSORS,
                (Move::ROCK, Outcome::DRAW) => Move::ROCK,
                (Move::ROCK, Outcome::WIN) => Move::PAPER,
                (Move::PAPER, Outcome::LOSS) => Move::ROCK,
                (Move::PAPER, Outcome::DRAW) => Move::PAPER,
                (Move::PAPER, Outcome::WIN) => Move::SCISSORS,
                (Move::SCISSORS, Outcome::LOSS) => Move::PAPER,
                (Move::SCISSORS, Outcome::DRAW) => Move::SCISSORS,
                (Move::SCISSORS, Outcome::WIN) => Move::ROCK,
            };
            let move_score: u32 = match my_move {
                Move::ROCK => 1,
                Move::PAPER => 2,
                Move::SCISSORS => 3,
            };
            move_score + my_move.score_vs(&self.them)
        }
    }

    impl Move {
        pub fn score_vs(&self, other: &Move) -> u32 {
            match (self, other) {
                (Move::ROCK, Move::ROCK) => 3,
                (Move::ROCK, Move::PAPER) => 0,
                (Move::ROCK, Move::SCISSORS) => 6,
                (Move::PAPER, Move::ROCK) => 6,
                (Move::PAPER, Move::PAPER) => 3,
                (Move::PAPER, Move::SCISSORS) => 0,
                (Move::SCISSORS, Move::ROCK) => 0,
                (Move::SCISSORS, Move::PAPER) => 6,
                (Move::SCISSORS, Move::SCISSORS) => 3,
            }
        }
    }
}
