use std::collections::HashMap;

use log::info;

pub fn solve_part_1(input: &str) {
    info!("marker: {}", start_of_marker(input, 4))
}

pub fn solve_part_2(input: &str) {
    info!("marker: {}", start_of_marker(input, 14))
}

fn start_of_marker(input: &str, num_chars: usize) -> usize {
    let mut map: HashMap<char, u32> = HashMap::new();
    let chars: Vec<char> = input.chars().collect();

    // range and update
    for i in 0..chars.len() {
        if i >= num_chars {
            match chars.get(i-num_chars) {
                Some(c) => match map.get_mut(c) {
                    Some(count) => {
                        *count -= 1;
                        if *count == 0 {
                            map.remove(c);
                        }
                    }
                    None => (),
                },
                None => (),
            }
        }
        
        *map.entry(chars[i]).or_default() += 1;

        if map.len() == num_chars {
            return i + 1;
        }
    }

    chars.len()
}