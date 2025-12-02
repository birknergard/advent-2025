use std::fs;

fn main() {
    println!("Hello, day one!");
    day_one();
}

struct Dial {
    current: usize,
    track: Vec<usize>,
    len: usize,
}

fn create_dial(size: usize) -> Dial {
    let mut dial: Vec<usize> = Vec::new();
    for i in 0..size {
        dial.push(i);
        //print!("{i} ");
    }
    println!();
    Dial {
        current: 0,
        len: dial.len(),
        track: dial,
    }
}
impl Dial {
    fn current_value(&self) -> &usize {
        &self.track[self.current]
    }

    fn max(&self) -> usize {
        self.track.len() - 1
    }

    fn min(&self) -> usize {
        0
    }

    fn set_current(&mut self, n: usize) {
        self.current = n;
    }

    // Returns whether it passed/landed on zero or not
    fn turn(&mut self, count: isize) -> u32 {
        // Going right
        let mut clicked: u32 = 0;
        if 0 < count {
            for _ in 0..count {
                if self.current + 1 > self.max() {
                    self.current = 0;
                    clicked += 1;
                } else {
                    self.current += 1;
                }
            }
        } else if count < 0 {
            let count = count * -1;
            for _ in 0..count {
                if self.current as isize - 1 < self.min() as isize {
                    self.current = self.max();
                } else {
                    self.current -= 1;
                }

                if self.current == 0 {
                    clicked += 1;
                }
            }
        }

        clicked
    }
}

fn day_one() {
    // Read from file
    let source = fs::read_to_string("/home/gizu/work/advent-of-code/day1/assets/dayone.txt")
        .expect("Should have been able to read contents of file.");

    // Convert to array of strings
    let source: Vec<&str> = Vec::from_iter(source.split_ascii_whitespace());

    // Create password var
    let mut password: u32 = 0;

    // Create and set dial
    let mut dial = create_dial(100);
    dial.set_current(50);
    println!("Set dial to {}", dial.current_value());

    // Handle code parse
    for code in source {
        let code = code.trim();

        let count: isize = code[1..].parse().expect("Could not parse numbers");
        let count = if code.starts_with('R') {
            count
        } else {
            count * -1 // Turn negative
        };

        print!(
            "{code}: rotated {count} times, from {}",
            dial.current_value()
        );
        let clicked = dial.turn(count);
        print!(" to {} ->", dial.current_value());

        if clicked > 0 {
            password += &clicked;
            println!(" clicked {clicked} times!");
        } else {
            println!("");
        }
    }

    println!("The password is {password}");
}

#[cfg(test)]
mod test {
    use crate::create_dial;

    #[test]
    fn track_is_correct() {
        let dial = create_dial(3);

        for i in 0..3 {
            if dial.track[i] != i {
                assert!(false)
            }
        }
        assert_eq!(dial.track.len(), 3)
    }

    #[test]
    fn can_turn_dial() {
        let mut dial = create_dial(3);
        dial.turn(2);
        assert_eq!(dial.current, 2)
    }

    #[test]
    fn can_overturn_dial_right() {
        let mut dial = create_dial(3);
        dial.turn(3);
        assert_eq!(dial.current, 0)
    }

    #[test]
    fn can_overturn_dial_left() {
        let mut dial = create_dial(3);
        dial.turn(-1);
        assert_eq!(dial.current, 2)
    }

    #[test]
    fn turn_back_then_forwards() {
        let mut dial = create_dial(3);
        dial.turn(-1);
        dial.turn(1);
        assert_eq!(dial.current, 0)
    }

    #[test]
    fn turn_around_twice() {
        let mut dial = create_dial(3);
        dial.turn(-3);
        dial.turn(-3);
        assert_eq!(dial.current, 0)
    }

    #[test]
    fn several_rotations() {
        let mut dial = create_dial(100);
        dial.set_current(50);
        let clicks = dial.turn(1000);
        assert_eq!(clicks, 10);
    }
}
