use std::fs::read_to_string;

fn main() {
    println!("Hello, day three!");
    let source: String = read_to_string("/home/gizu/work/advent-of-code/day3/daythree.txt")
        .expect("Could not read file.");

    //part1(&source);
    part2(&source);
}

fn part1(source: &String) {
    let banks = source.split('\n');
    let mut solution: u32 = 0;

    for bank in banks {
        // FIX: Weird error
        if bank.is_empty() {
            continue;
        }
        //let bank = bank.trim();
        let mut set: [u8; 2] = [0; 2];
        let mut position: usize = 0;

        // Find highest value before last index
        for (i, battery) in bank.chars().enumerate() {
            let battery: u8 = (battery.to_digit(10).expect("Could not parse digit.")) as u8;

            // Stop when we hit the highest possible digit or next to end of range
            if set[0] == 9 || i == bank.len() - 1 {
                break;
            }

            // Update position and value when we find a higher one than current
            if set[0] < battery {
                set[0] = battery;
                position = i;
            }
        }

        // Split bank on that index, check rest of bank for highest digit
        println!("Splitting at {} of {}", position, bank.len());
        let bank_remainder = &bank[position + 1..];
        print!("Remainder: {:?}", bank_remainder);
        for (_, battery) in bank_remainder.chars().enumerate() {
            let battery: u8 = (battery.to_digit(10).expect("Could not parse digit.")) as u8;
            if set[1] < battery {
                set[1] = battery;
                // Stop if we hit the highest possible digit
                if set[1] == 9 {
                    break;
                }
            }
        }

        println!("Voltage for battery is {}{}", set[0], set[1]);
        let voltage = set[0] * 10 + set[1];
        solution += voltage as u32;
    }
    println!("Part 1 Solution = {solution}");
}

fn part2(source: &String) {
    let banks = source.split('\n');
    let mut solution: u64 = 0;

    for (line, bank) in banks.enumerate() {
        // FIX: Weird error
        if bank.is_empty() {
            continue;
        }
        {
            // Remove i+=1 sequentially until the length of the array is 12
            let bank: Vec<char> = bank.chars().collect();
            let mut joltage: Vec<char> = Vec::new();

            // Find the largest number first (until length i)
            // Delete until the largest numbers index
            let mut max_index = 0;
            let mut sliced = &bank[..];

            // Go from highest digit to lowest
            for battery_digit in (0..=11).rev() {
                // Get new range after first until checked all
                if battery_digit < 11 {
                    if max_index + 1 < sliced.len() {
                        // Omits first character in slice as it was already stored
                        sliced = &sliced[max_index + 1..];
                    }
                }

                // Find highest numbers(index) in sliced range, max current char (battery_digit)
                max_index = 0;
                let mut max_value = 0;
                for (i, battery) in sliced.iter().enumerate() {
                    // Convert from char to int
                    let battery = battery.to_digit(10).expect("Couldn't convert char to int");

                    // Stop at counter first occurence of highest digit
                    if max_value < battery {
                        max_index = i;
                        max_value = battery;
                    }

                    // Stop if hit battery_digit index
                    if i >= (sliced.len() - battery_digit) - 1 {
                        break;
                    }
                }

                // Add highest number and then check from next
                joltage.push(sliced[max_index]);
                {
                    let joltage = String::from_iter(&joltage);
                    let sliced = String::from_iter(sliced);
                    println!(
                        "Added {max_value} to {} for {}th digit in {}",
                        String::from(joltage),
                        battery_digit + 1,
                        String::from(sliced)
                    );
                }
            }

            let max_joltage: u64 = vec_char_to_num(&joltage);
            solution += max_joltage;
            println!("{max_joltage}, {}", String::from_iter(bank));
        }
        //println!("{}", bank);
    }
    println!("Part 2 Solution = {solution}");
}

fn vec_char_to_num(vec: &Vec<char>) -> u64 {
    if vec.len() != 12 {
        panic!(
            "Invalid number given, needs to be 12 length, is {}",
            vec.len()
        );
    }
    let mut sum: u64 = 0;
    let len = vec.len();

    for (i, item) in vec.iter().enumerate() {
        let digit = item.to_digit(10).expect("Could not convert into digit");
        // Calculate the correct power of 10 based on the position from the end
        let exponent: u64 = 10u64.pow((len - 1 - i) as u32);
        sum += digit as u64 * exponent;
    }

    sum
}
