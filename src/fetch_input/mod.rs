#![allow(unused)]
use std::{env, fmt::Display, fs};

pub struct InputFetcher {
    client: reqwest::blocking::Client,
}

impl InputFetcher {
    pub fn new() -> Self {
        dotenv::dotenv();
        InputFetcher {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_input(&self, y: impl Display, d: impl Display) -> String {
        read_from_file(&y, &d)
            .or_else(|| self.fetch_remote_and_cache(&y, &d))
            .unwrap()
    }

    fn fetch_remote_and_cache(&self, y: &impl Display, d: &impl Display) -> Option<String> {
        println!("Fetching from remote.");
        let url = format!("https://adventofcode.com/{}/day/{}/input", y, d);
        let session = format!("session={}", env::var("AOC_SESSION_TOKEN").unwrap());

        let input = self
            .client
            .get(url)
            .header("Cookie", session)
            .header(
                "User-Agent",
                "https://github.com/nmhanson/aoc2022 by nathanmhanson@icloud.com",
            )
            .send()
            .ok()
            .filter(|resp| resp.status().is_success())
            .and_then(|resp| resp.text().ok());

        if let Some(i) = input.as_ref() {
            println!("Writing to file {}", file_path(y, d));
            fs::create_dir_all(file_path(y, d)).unwrap();
            fs::write(format!("{}/{}", file_path(y, d), "input"), i).unwrap();
        }
        input
    }
}

fn read_from_file(y: &impl Display, d: &impl Display) -> Option<String> {
    fs::read_to_string(format!("{}/{}", file_path(y, d), "input")).ok()
}

fn file_path(y: &impl Display, d: &impl Display) -> String {
    format!("./input_cache/y{}/d{}", y, d)
}
