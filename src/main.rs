mod fetch_input;
mod y2022;
use fetch_input::InputFetcher;
fn main() {
    dotenv::dotenv().ok();
    let in_fetcher = InputFetcher::new();
    let input = in_fetcher.fetch_input("2022", "2");
    y2022::day_two::part_one(&input);
    y2022::day_two::part_two(&input);
}
