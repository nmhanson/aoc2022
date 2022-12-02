use rps::*;
use std::str::FromStr;

pub fn part_one(input: &str) {
    let total: u32 = input
        .trim()
        .split('\n')
        .map(|rnd_str| RPSRound::from_str(rnd_str).unwrap())
        .map(|round| round.score())
        .sum();

    println!("part_one: {}", total);
}

mod rps {
    use std::str::FromStr;

    pub struct RPSRound {
        me: Move,
        them: Move,
    }

    #[derive(Debug)]
    enum Move {
        ROCK,
        PAPER,
        SCISSORS,
    }

    impl FromStr for RPSRound {
        type Err = ParseRoundError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut tokens = s.split(' ');
            let opponent = tokens.next();
            let me = tokens.next();
            match (me, opponent) {
                (Some(m), Some(o)) => Ok(RPSRound {
                    me: Move::from_str(m)?,
                    them: Move::from_str(o)?,
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

    #[derive(Debug)]
    pub struct ParseRoundError(&'static str);

    impl RPSRound {
        pub fn score(&self) -> u32 {
            let move_score: u32 = match self.me {
                Move::ROCK => 1,
                Move::PAPER => 2,
                Move::SCISSORS => 3,
            };
            move_score + self.me.score_vs(&self.them)
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
