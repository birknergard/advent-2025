use std::fs::read_to_string;

fn main() {
    println!("Hello, day five!");
    let file = read_to_string("/home/gizu/work/advent-of-code/day5/input.txt")
        .expect("Could not read puzzle input.");
    let file = file.split_once("\n\n");

    match file {
        Some((ranges, ids)) => {
            let ranges: Vec<&str> = ranges.split_terminator('\n').collect();
            let ids: Vec<&str> = ids.split_terminator('\n').collect();
            part_one(ranges, ids);
        }
        None => (),
    }
}

fn part_one(ranges: Vec<&str>, ids: Vec<&str>) {
    for range in ranges {
        let range: Vec<u128> = range.split_terminator('-').collect();
        let range_min = range[0];
        let range_max = range[1];
    }
}

fn part_two() {}
