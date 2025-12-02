use std::{
    fs::{self, read_to_string},
    str::Chars,
};

fn main() {
    println!("Hello, day two!");
    let source = read_to_string("/home/gizu/work/advent-of-code/day2/assets/daytwo.txt")
        .expect("Could not open puzzle input.");

    let source = source.split(',');
    let mut solution: u64 = 0;

    // Split each string in two, check if they are the same
    for id in source {
        let id = id.trim();
        let range: Vec<&str> = Vec::from_iter(id.split('-'));
        let mut range_nums: [u32; 2] = [0; 2];

        for i in 0..range.len() {
            range_nums[i] = range[i].parse().expect("Could not convert value");
        }

        // Brute force puzzle solution
        for id in range_nums[0]..range_nums[1] + 1 {
            // Only check even ids
            let id_str = id.clone().to_string();

            let mut part: Vec<char> = Vec::new();

            // Loops for each number in id
            let mut invalid = false;
            for (i, id_part) in id_str.chars().enumerate() {
                // Always store first num
                if i == 0 {
                    part.push(id_part);
                    continue;
                }

                // Comparison logic starts
                if part[0] == id_part {
                    // Split the string into part.len() size parts
                    // If the part is not a remainder of the max length the part string is invalid
                    let split = split_every_x(&id_str, part.len());

                    // Compare each split with the previous
                    for (y, char) in split.iter().enumerate() {
                        if y == 0 {
                            continue;
                        }

                        if *char != split[0] {
                            //println!("is ok!");
                            break;
                        }
                        // If all comparisons pass the id is invalid
                        if y == split.len() - 1 {
                            println!("\n{id} - {:?}", split);
                            invalid = true;
                            break;
                        }
                    }
                }
                if invalid {
                    break;
                }

                // Push character to id string
                part.push(id_part);
            }
            if invalid {
                println!("Found invalid code!");
                solution += id as u64;
            }
        }
    }
    println!("Solution: {}", solution);
}

fn split_every_x(list: &str, size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;

    while start < list.len() {
        let end = (start + size).min(list.len());
        result.push(list[start..end].to_string());
        start += size;
    }

    result
}
