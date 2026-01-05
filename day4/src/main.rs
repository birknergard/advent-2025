use std::fs::read_to_string;

fn main() {
    println!("Hello, day four!");
    let file = read_to_string("/home/gizu/work/advent-of-code/day4/input.txt")
        .expect("Could not read puzzle input.");

    let mut m: Vec<Vec<char>> = file
        .split_terminator('\n')
        .map(|row| Vec::from_iter(row.chars()))
        .collect();

    let solution = part_two(&mut m);
    println!("Solution: {solution} rows of toilet paper rolls can be moved.");
}

fn part_one(input: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut movable_roll_positions: Vec<(usize, usize)> = Vec::new();

    for i in 0..input.len() {
        let curr_row = &input[i];

        for k in 0..curr_row.len() {
            let mut rolls = 0;
            if curr_row[k] == '@' {
                // check adjacent right
                let right = curr_row.get(k + 1);
                match right {
                    Some(right) => {
                        if *right == '@' {
                            rolls += 1;
                        }
                    }
                    None => (),
                }

                // check adjacent left
                if k > 0 {
                    let left = curr_row.get(k - 1);
                    match left {
                        Some(left) => {
                            if *left == '@' {
                                rolls += 1;
                            }
                        }
                        None => (),
                    }
                }

                // check row above
                if i > 0 {
                    let top_row = input.get(i - 1);
                    match top_row {
                        Some(row) => {
                            rolls += get_rolls_in_row_slice(&row, &k);
                        }
                        None => (),
                    }
                }

                // check row below
                let bot_row = input.get(i + 1);
                match bot_row {
                    Some(row) => {
                        rolls += get_rolls_in_row_slice(&row, &k);
                    }
                    None => (),
                }
                if rolls < 4 {
                    movable_roll_positions.push((i, k));
                }
            }
        }
    }
    movable_roll_positions
}

fn part_two(input: &mut Vec<Vec<char>>) -> usize {
    let mut rolls_moved = 0;
    loop {
        let movable_rolls_positions = part_one(input);
        println!(
            "{} rolls can be accessed by forklift!",
            movable_rolls_positions.len()
        );
        if movable_rolls_positions.is_empty() {
            break;
        }

        // 'move' the rolls
        rolls_moved += movable_rolls_positions.len();
        for position in movable_rolls_positions {
            input[position.0][position.1] = '.';
        }
    }
    rolls_moved
}

fn get_rolls_in_row_slice(row: &Vec<char>, from_index: &usize) -> usize {
    let mut rolls: usize = 0;
    // Check left
    if *from_index > 0 {
        if row[*from_index - 1] == '@' {
            rolls += 1;
        }
    }

    // check middle
    if row[*from_index] == '@' {
        rolls += 1;
    }

    // Check right
    if *from_index < row.len() - 1 {
        if row[*from_index + 1] == '@' {
            rolls += 1;
        }
    }

    rolls
}
