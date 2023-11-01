use std::collections::BinaryHeap;

use log::info;

pub fn solve_part_1(input: &str) {
    let elf_inventory = elf_calories(input);
    info!("calories (1): {}", most_calories(elf_inventory, 1));
}

pub fn solve_part_2(input: &str) {
    let elf_inventory = elf_calories(input);
    info!("calories (3): {}", most_calories(elf_inventory, 3));
}

fn elf_calories(calorie_list: &str) -> BinaryHeap<u32> {
    let mut heap = BinaryHeap::new();
    let mut calories = 0;
    for line in calorie_list.lines() {
        if line.is_empty() {
            heap.push(calories);
            calories = 0;
            continue;
        }
        calories += line.parse::<u32>().expect("input could not be parsed to a u32");
    }
    heap.push(calories);
    heap
}

fn most_calories(elf_calories: BinaryHeap<u32>, elves: usize) -> u32 {
    elf_calories.into_sorted_vec().iter().rev().take(elves).sum()
}