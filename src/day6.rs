#![allow(unused)]
mod part_one {
    use std::collections::HashSet;

    pub fn solution(input: &str) {
        let chars = input.chars().collect::<Vec<_>>();
        let first_four_unique = chars
            .windows(4)
            .position(|window| window.iter().map(|c| c).collect::<HashSet<_>>().len() == 4)
            .unwrap();

        println!("part_one: {}", first_four_unique + 4)
    }
}

mod part_two {
    use std::collections::HashSet;

    pub fn solution(input: &str) {
        let chars = input.chars().collect::<Vec<_>>();
        let first_fourteen_unique = chars
            .windows(14)
            .position(|window| window.iter().map(|c| c).collect::<HashSet<_>>().len() == 14)
            .unwrap();

        println!("part_two: {}", first_fourteen_unique + 14)
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 6);
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
