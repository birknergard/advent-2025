use std::fs::read_to_string;

fn main() {
    println!("Hello, day three!");
    let source = read_to_string("/home/gizu/work/advent-of-code/day3/daythree.txt")
        .expect("Could not read file.");
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
    println!("Solution = {}", solution);
}
