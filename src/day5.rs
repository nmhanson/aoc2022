#![allow(unused)]
mod part_one {
    use std::str::FromStr;

    use super::crates::Input;
    pub fn solution(input: &str) {
        let mut input = Input::from_str(input).unwrap();

        for mv in input.moves.iter() {
            for _ in 0..mv.amt {
                let c = input.crates[mv.from - 1].pop().unwrap();
                input.crates[mv.to - 1].push(c);
            }
        }
        let tops = input
            .crates
            .iter()
            .map(|stack| stack.last().unwrap().to_owned())
            .fold(String::new(), |res, c| format!("{}{}", res, c));
        println!("part_one: {}", tops)
    }
}

mod part_two {
    use std::str::FromStr;

    use super::crates::Input;
    pub fn solution(input: &str) {
        let mut input = Input::from_str(input).unwrap();

        for mv in input.moves.iter() {
            let mut crates: Vec<char> = vec![];
            for _ in 0..mv.amt {
                crates.insert(0, input.crates[mv.from - 1].pop().unwrap());
            }
            input.crates[mv.to - 1].append(&mut crates);
        }
        let tops = input
            .crates
            .iter()
            .map(|stack| stack.last().unwrap().to_owned())
            .fold(String::new(), |res, c| format!("{}{}", res, c));
        println!("part_two: {}", tops)
    }
}

mod crates {
    use anyhow::{anyhow, Error, Ok};
    use std::str::FromStr;

    pub struct Input {
        pub crates: Vec<Vec<char>>,
        pub moves: Vec<Move>,
    }

    impl FromStr for Input {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split("\n\n");
            let crates_input = parts.next().unwrap();
            let moves_input = parts.next().unwrap();

            Ok(Input {
                crates: parse_crate_input(crates_input),
                moves: parse_moves_input(moves_input),
            })
        }
    }

    fn parse_crate_input(s: &str) -> Vec<Vec<char>> {
        let rows = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .map(|chars| chars_to_crates(&chars))
            .collect::<Vec<_>>();

        let mut stacks: Vec<Vec<char>> = vec![];
        for r in rows.iter() {
            stacks.push(vec![]);
        }
        for r in rows.iter() {
            for (i, s) in r.iter().enumerate() {
                if let Some(c) = s {
                    stacks[i].insert(0, c.to_owned());
                }
            }
        }
        stacks
    }

    fn chars_to_crates(chars: &Vec<char>) -> Vec<Option<char>> {
        chars
            .chunks(4)
            .map(|chunk| match chunk {
                ['[', c, ']', _] => Some(c.to_owned()),
                ['[', c, ']'] => Some(c.to_owned()),
                _ => None,
            })
            .collect()
    }
    #[derive(Debug)]
    pub struct Move {
        pub amt: usize,
        pub from: usize,
        pub to: usize,
    }

    fn parse_moves_input(s: &str) -> Vec<Move> {
        s.trim()
            .lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .map(|tokens| Move {
                amt: tokens[1].parse().unwrap(),
                from: tokens[3].parse().unwrap(),
                to: tokens[5].parse().unwrap(),
            })
            .collect()
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 5);
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
