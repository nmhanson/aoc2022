#![allow(unused)]
mod part_one {
    pub fn solution(input: &String) {
        let elvs = input.split("\n\n");
        let max_elf = elvs.map(sum_elf_calories).max();
        if let Some(max_elf) = max_elf {
            println!("part_one: {}", max_elf)
        }
    }

    fn sum_elf_calories(elf_list: &str) -> u32 {
        let items = elf_list
            .trim()
            .split('\n')
            .map(|n| n.parse::<u32>().unwrap());
        items.sum()
    }
}

mod part_two {
    pub fn solution(input: &String) {
        let elvs = input.split("\n\n").map(str::trim).map(split_parse_and_sum);

        let mut top_earners = [0, 0, 0];

        for e in elvs {
            for (i, v) in top_earners.into_iter().enumerate() {
                if e > v {
                    top_earners[i] = e;
                    break;
                }
            }
        }
        println!("part_two: {}", top_earners.iter().sum::<u32>());
    }

    pub fn split_parse_and_sum(elf: &str) -> u32 {
        elf.split('\n').map(|elf| elf.parse::<u32>().unwrap()).sum()
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two};

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 1);
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
