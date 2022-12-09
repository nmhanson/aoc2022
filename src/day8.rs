#![allow(unused)]
mod part_one {
    use super::trees::Forest;

    pub fn solution(input: &Forest) {
        let inner_tree_coords =
            (1..input.0.len() - 1).flat_map(|y| (1..input.0[0].len() - 1).map(move |x| (x, y)));
        let res = inner_tree_coords
            .filter(|&c| input.tree_is_visible(c))
            .count();
        let perimeter = (input.0.len() * 2) + (input.0[0].len() * 2) - 4;
        println!("part_one: {}", res + perimeter)
    }
}

mod part_two {
    use super::trees::Forest;

    pub fn solution(input: &Forest) {
        let inner_tree_coords =
            (1..input.0.len() - 1).flat_map(|y| (1..input.0[0].len() - 1).map(move |x| (x, y)));
        let res = inner_tree_coords
            .map(|c| input.viewing_score(c))
            .max()
            .unwrap();
        println!("part_two: {}", res)
    }
}

mod trees {
    use std::str::FromStr;

    use anyhow::{anyhow, Error, Result};

    pub struct Forest(pub Vec<Vec<usize>>);
    impl FromStr for Forest {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            let trees: Option<Vec<Vec<usize>>> = s
                .trim()
                .lines()
                .map(|row| {
                    row.trim()
                        .chars()
                        .map(|c| c.to_digit(10).map(|n| n as usize))
                        .collect()
                })
                .collect();

            Ok(Forest(trees.ok_or(anyhow!("Failed to parse input"))?))
        }
    }

    impl Forest {
        pub fn tree_is_visible(&self, (x, y): (usize, usize)) -> bool {
            let height = self.0[y][x];
            let forest_height = self.0.len();
            let forest_width = self.0[0].len();
            (0..x).all(|dx| self.0[y][dx] < height) // left
                || (x + 1..forest_width).all(|dx| self.0[y][dx] < height) // right
                || (0..y).all(|dy| self.0[dy][x] < height) // up
                || (y + 1..forest_height).all(|dy| self.0[dy][x] < height)
            //down
        }

        pub fn viewing_score(&self, (x, y): (usize, usize)) -> usize {
            let height = self.0[y][x];
            let forest_height = self.0.len();
            let forest_width = self.0[0].len();
            ((1..x).take_while(|&dx| self.0[y][x - dx] < height).count() + 1) // left
                * ((1..forest_width - x - 1).take_while(|&dx| self.0[y][x + dx] < height).count() + 1) // right
                * ((1..y).take_while(|&dy| self.0[y - dy][x] < height).count() + 1) // up
                * ((1..forest_height - y - 1).take_while(|&dy| self.0[y + dy][x] < height).count()+ 1)
            //down
        }
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::{part_one, part_two, trees::Forest};

    const EXAMPLE: &str = "
30373
25512
65332
33549
35390
    ";

    #[test]
    fn run() {
        let input = InputFetcher::new().fetch_input(2022, 8).parse().unwrap();
        part_one::solution(&input);
        part_two::solution(&input);
    }

    #[test]
    fn run_example() {
        let input = EXAMPLE.parse().unwrap();
        part_one::solution(&input);
        part_two::solution(&input);
    }
}
