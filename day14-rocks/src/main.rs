use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    None,
}

type Platform = Vec<Vec<Rock>>;

fn parse_platform(platform: &str) -> Platform {
    platform
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'o' | 'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn shift_and_rotate(platform: &mut Platform) {
    let len = platform.len();
    let mut shifted: Platform = Vec::with_capacity(len);

    for x in 0..len {
        let mut row = Vec::with_capacity(len);
        let mut empties = 0;
        let mut rounds = 0;

        for y in 1..=len {
            let rock = platform[len - y][x];
            match rock {
                Rock::Round => rounds += 1,
                Rock::None => empties += 1,
                Rock::Cube => {
                    row.extend(vec![Rock::None; empties]);
                    row.extend(vec![Rock::Round; rounds]);
                    row.push(Rock::Cube);
                    empties = 0;
                    rounds = 0;
                }
            }
        }
        row.extend(vec![Rock::None; empties]);
        row.extend(vec![Rock::Round; rounds]);
        shifted.push(row);
    }

    *platform = shifted
}

fn count_north_weight(platform: &Platform) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, column)| (idx + 1) * column.iter().filter(|&rock| *rock == Rock::Round).count())
        .sum()
}

fn hash_platform(platform: &Platform) -> u64 {
    let mut hasher = DefaultHasher::new();
    platform.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let now = std::time::Instant::now();
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let document =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut loop_detector = HashMap::new();
    let mut platform = parse_platform(&document);
    let mut rotations_left = 1_000_000_000_u32;
    let mut i = 0;

    while rotations_left > 0 {
        for _ in 0..4 {
            shift_and_rotate(&mut platform)
        }
        rotations_left -= 1;
        i += 1;

        if let Some(original_idx) = loop_detector.insert(hash_platform(&platform), i) {
            rotations_left %= i - original_idx;
        }
    }
    let result = count_north_weight(&platform);

    println!("Result: {}", result);

    println!("Runtime: {:?}", now.elapsed());
}
