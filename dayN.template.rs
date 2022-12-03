#![allow(unused)]
mod part_one {
    pub fn solution(input: &str) {
        todo!()
    }
}

mod part_two {
    pub fn solution(input: &str) {
        todo!()
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(-1, -1); // Set year & day
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
