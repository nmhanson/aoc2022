#![allow(unused)]

use std::{ops::Range, str::FromStr};
mod part_one {
    use std::str::FromStr;

    use super::pairs::ElfPair;

    pub fn solution(input: &str) {
        let result = input
            .trim()
            .split('\n')
            .map(ElfPair::from_str)
            .filter_map(Result::ok)
            .filter(pair_is_redundant)
            .count();

        println!("part_one: {}", result)
    }

    fn pair_is_redundant(ElfPair(e1, e2): &ElfPair) -> bool {
        (e1.0 >= e2.0 && e1.1 <= e2.1) || (e2.0 >= e1.0 && e2.1 <= e1.1)
    }
}

mod part_two {
    use std::str::FromStr;

    use super::pairs::ElfPair;
    pub fn solution(input: &str) {
        let result = input
            .trim()
            .split('\n')
            .map(ElfPair::from_str)
            .filter_map(Result::ok)
            .filter(pair_has_overlap)
            .count();

        println!("part_two: {}", result)
    }

    fn pair_has_overlap(ElfPair(e1, e2): &ElfPair) -> bool {
        e1.0 <= e2.1 && e2.0 <= e1.1
    }
}

mod pairs {
    use std::str::FromStr;

    use anyhow::{anyhow, Error};
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref PAIR_RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    }

    #[derive(Debug)]
    pub struct ElfPair(pub (usize, usize), pub (usize, usize));

    impl FromStr for ElfPair {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let caps = PAIR_RE
                .captures(s)
                .ok_or(anyhow!("Failed to match regex on str {}", s))?;

            let mut parsed_caps = caps
                .iter()
                .skip(1)
                .filter_map(|c| c.and_then(|m| m.as_str().parse::<usize>().ok()));

            let elves = (
                parsed_caps.next(),
                parsed_caps.next(),
                parsed_caps.next(),
                parsed_caps.next(),
            );

            match elves {
                (Some(c1), Some(c2), Some(c3), Some(c4)) => Ok(Self((c1, c2), (c3, c4))),
                _ => Err(anyhow!("Failed to parse all capture groups")),
            }
        }
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 4);
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
