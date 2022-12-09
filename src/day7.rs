#![allow(unused)]
mod part_one {

    use std::fs;

    use super::file_system::*;
    use super::input::{is_ls_output, InputLineType};

    pub fn solution(input: &FSArenaTree) -> usize {
        let res: usize = input
            .0
            .iter()
            .filter_map(|(_, node)| node.directory())
            .map(|node| input.get_size_of_dir(node.path.clone()))
            .filter(|&size| size <= 100000)
            .sum();

        println!("part_one: {}", res);
        res
    }
}

mod part_two {

    use std::fs;

    use super::file_system::*;
    use super::input::{is_ls_output, InputLineType};

    pub fn solution(input: &FSArenaTree) -> usize {
        let needed_space = 30_000_000_usize;
        let total_space = 70_000_000_usize;
        let free_space =
            total_space - input.get_size_of_dir(input.0.get("/".into()).unwrap().path.clone());

        let min_needed_new_space = needed_space - free_space;

        let res: usize = input
            .0
            .iter()
            .filter_map(|(_, node)| node.directory())
            .map(|node| input.get_size_of_dir(node.path.clone()))
            .filter(|&size| size >= min_needed_new_space)
            .min()
            .unwrap();

        println!("part_two: {}", res);
        res
    }
}

mod input {
    use anyhow::{bail, Error, Ok};
    use std::str::FromStr;

    impl FromStr for InputLineType {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tokens: Vec<&str> = s.trim().split_whitespace().collect();
            match tokens[..] {
                ["$", "cd", s] => Ok(InputLineType::CDCommand(s.to_string())),
                ["$", "ls"] => Ok(InputLineType::LSCommand),
                ["dir", n] => Ok(InputLineType::Dir(n.to_string())),
                [s, n] if s.parse::<usize>().is_ok() => {
                    Ok(InputLineType::File(n.to_string(), s.parse().unwrap()))
                }
                _ => bail!("Input parsing encountered unknown pattern {}", s),
            }
        }
    }

    #[derive(Debug)]
    pub enum InputLineType {
        CDCommand(String),
        LSCommand,
        File(String, usize),
        Dir(String),
    }

    pub fn is_ls_output(i: &InputLineType) -> bool {
        match i {
            InputLineType::File(_, _) => true,
            InputLineType::Dir(_) => true,
            _ => false,
        }
    }
}

mod file_system {
    use std::{
        collections::{HashMap, HashSet},
        fmt::Debug,
        fs,
        path::Path,
        str::FromStr,
    };

    use anyhow::{anyhow, bail, Error, Ok, Result};

    use super::input::{is_ls_output, InputLineType};

    #[derive(Clone, Copy, Debug)]
    pub enum FSObject {
        File(usize),
        Dir,
    }

    #[derive(Clone, Debug)]
    pub struct FSNode {
        pub path: String,
        parent: Option<String>,
        data: FSObject,
        children: HashSet<String>,
    }

    impl FSNode {
        pub fn directory(&self) -> Option<&Self> {
            match self.data {
                FSObject::File(_) => None,
                FSObject::Dir => Some(self),
            }
        }
    }

    #[derive(Debug)]
    pub struct FSArenaTree(pub HashMap<String, FSNode>);

    impl FSArenaTree {
        pub fn new(root: String) -> Self {
            let root_node = FSNode {
                path: root.clone(),
                parent: None,
                data: FSObject::Dir,
                children: HashSet::new(),
            };
            FSArenaTree(HashMap::from([(root, root_node)]))
        }

        fn node(&mut self, path: String, data: FSObject) -> String {
            if let Some(node) = self.0.get(&path) {
                path
            } else {
                let node = FSNode {
                    path: path.clone(),
                    parent: None,
                    data,
                    children: HashSet::new(),
                };

                self.0.insert(path.clone(), node);
                path
            }
        }

        pub fn get_size_of_dir(&self, dir_path: String) -> usize {
            let d = self.0.get(&dir_path).unwrap();
            d.children
                .iter()
                .map(|child| {
                    let child = self.0.get(child).unwrap();
                    match child.data {
                        FSObject::File(size) => size,
                        FSObject::Dir => self.get_size_of_dir(child.path.clone()),
                    }
                })
                .sum()
        }

        pub fn add_child(
            &mut self,
            parent: String,
            name: String,
            data: FSObject,
        ) -> Result<String> {
            let node_path = self.node(format!("{}/{}", parent, name), data);

            let parent_node = self
                .0
                .get_mut(&parent)
                .ok_or(anyhow!("attempted to add child to non-existent path"))?;

            parent_node.children.insert(node_path.clone());

            let child_node = self.0.get_mut(&node_path).unwrap();
            child_node.parent = Some(parent);
            Ok(node_path)
        }
    }

    impl FromStr for FSArenaTree {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut fs_tree = FSArenaTree::new("/".to_string());
            let mut input_lines = s
                .trim()
                .lines()
                .map(str::parse::<InputLineType>)
                .flat_map(Result::into_iter)
                .peekable();

            let mut current_path: Vec<String> = vec![];
            while let Some(l) = input_lines.next() {
                match l {
                    InputLineType::CDCommand(n) => {
                        if n == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(n.clone());
                        }
                    }
                    InputLineType::LSCommand => {
                        while input_lines.peek().filter(|&l| is_ls_output(l)).is_some() {
                            match input_lines.next().unwrap() {
                                InputLineType::File(n, s) => {
                                    let path_str = current_path.join("/");
                                    fs_tree.add_child(path_str, n, FSObject::File(s));
                                }
                                InputLineType::Dir(n) => {
                                    let path_str = current_path.join("/");
                                    fs_tree.add_child(path_str, n, FSObject::Dir);
                                }
                                _ => bail!("this was supposed to be an ls item"),
                            }
                        }
                    }
                    _ => bail!("ls item not expected here"),
                };
            }
            Ok(fs_tree)
        }
    }
}

#[cfg(test)]
mod run {
    use crate::fetch_input::InputFetcher;

    use super::file_system::*;
    use super::{part_one, part_two};

    const EXAMPLE_INPUT: &'static str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    ";

    #[test]
    fn run() {
        let input = InputFetcher::new()
            .fetch_input(2022, 7)
            .parse::<FSArenaTree>()
            .unwrap(); // Set year & day
        part_one::solution(&input);
        part_two::solution(&input);
    }

    #[test]
    fn run_example() {
        let input = EXAMPLE_INPUT.trim().parse::<FSArenaTree>().unwrap();
        assert_eq!(part_one::solution(&input), 95437_usize);
        assert_eq!(part_two::solution(&input), 24933642_usize);
    }
}
