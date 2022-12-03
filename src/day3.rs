#![allow(unused)]
mod part_one {
    use super::rucksack::{self, convert_character_to_priority, Rucksack};
    use anyhow::Result;
    use std::{any, collections::HashSet, str::FromStr};

    pub fn solution(input: &str) {
        let result: u32 = input
            .trim()
            .split('\n')
            .map(Rucksack::from_str)
            .map(|rucksack| rucksack.and_then(get_common_item_value))
            .map(Result::unwrap)
            .sum();
        println!("part_one: {}", result)
    }

    fn get_common_item_value(rucksack: Rucksack) -> Result<u32> {
        let (c1, c2) = rucksack.0.split_at(rucksack.0.len() / 2);
        let c1: HashSet<_> = c1.chars().collect();
        let c2: HashSet<_> = c2.chars().collect();
        let common_item = c1.intersection(&c2).next();
        common_item
            .map(convert_character_to_priority)
            .ok_or(anyhow::anyhow!("Failed to find common item."))
    }
}

mod part_two {
    use std::{collections::HashSet, str::FromStr, vec};

    use super::rucksack::{self, Rucksack};

    pub fn solution(input: &str) {
        let mut rs = input
            .trim()
            .split('\n')
            .map(Rucksack::from_str)
            .map(Result::unwrap);

        let mut common_items: Vec<u32> = vec![];
        while let (Some(a), Some(b), Some(c)) = (rs.next(), rs.next(), rs.next()) {
            let (a, b, c): (HashSet<char>, HashSet<_>, HashSet<_>) = (
                a.0.chars().collect(),
                b.0.chars().collect(),
                c.0.chars().collect(),
            );

            let a_b: HashSet<_> = a.intersection(&b).map(char::to_owned).collect();
            let mut common = a_b.intersection(&c);
            let badge = common.next().unwrap();
            common_items.push(rucksack::convert_character_to_priority(badge));
        }
        let result: u32 = common_items.iter().sum();

        println!("part_two: {}", result)
    }
}

mod rucksack {
    use std::str::FromStr;

    use anyhow::Ok;

    pub struct Rucksack(pub String);

    impl FromStr for Rucksack {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Rucksack(s.into()))
        }
    }

    pub fn convert_character_to_priority(c: &char) -> u32 {
        let subtrahend = if c.is_uppercase() { 38 } else { 96 };
        (*c as u32) - subtrahend
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 3);

        part_one::solution(&input);
        part_two::solution(&input);
    }
}
