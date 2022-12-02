pub fn part_one(input: &String) {
    let elvs = input.split("\n\n");
    let max_elf = elvs.map(sum_elf_calories).max();
    if let Some(max_elf) = max_elf {
        println!("max_elf: {}", max_elf)   
    }
}

fn sum_elf_calories(elf_list: &str) -> u32 {
    let items = elf_list.trim().split('\n').map(|n| n.parse::<u32>().unwrap());
    items.sum()
}
