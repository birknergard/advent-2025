use std::fs::read_to_string;

fn main() {
    println!("Hello, day four!");
    let file = read_to_string("/home/gizu/work/advent-of-code/day4/input.txt")
        .expect("Could not puzzle input.");

    part_one(&file);
}

fn part_one(input: &String) {
    let m: Vec<Vec<char>> = input
        .split_terminator('\n')
        .map(|row| Vec::from_iter(row.chars()))
        .collect();
    let rows = m.len();
    let cols = m[0].len();
    println!("rows {rows}, cols {cols}");
    let mut moveable_rolls = 0;

    for i in 0..rows {
        let top = Some(m.get(i - 1));
        let bot = Some(m.get(i + 1));
        for k in 0..cols {
            if m[i][k] == '@' {
                let mut adjacent = 0;

                // Check top
                if i > 0 {
                    for a in 0..3 {
                        if m[i - 1][a + k - 1] == '@' {
                            adjacent += 1;
                        }
                    }
                }
                // Check bottom
                if i < rows {
                    for a in 0..3 {
                        if m[i + 1][a + k - 1] == '@' {
                            adjacent += 1;
                        }
                    }
                }

                // Check left
                if k > 0 {
                    if m[i][k - 1] == '@' {
                        adjacent += 1;
                    }
                }

                // Check right
                if k < cols {
                    if m[i][k + 1] == '@' {
                        adjacent += 1;
                    }
                }

                if adjacent < 4 {
                    moveable_rolls += 1;
                }
            }
        }
    }
    println!("Sol: {moveable_rolls} rolls can be accessed by forklift!");
}
