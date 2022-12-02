pub fn part_two(input: &String) {
    let elvs = input
        .split("\n\n")
        .map(str::trim)
        .map(split_parse_and_sum);

    let mut top_earners = [0, 0, 0];

    for e in elvs {
      for (i, v) in top_earners.into_iter().enumerate() {
        if e > v {
          top_earners[i] = e;
          break;
        }
      }
    }
    println!("top three: {}", top_earners.iter().sum::<u32>());
}

pub fn split_parse_and_sum(elf: &str) -> u32 {
    elf.split('\n').map(|elf| elf.parse::<u32>().unwrap()).sum()
}
