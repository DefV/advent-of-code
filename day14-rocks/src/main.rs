use std::collections::{HashMap, hash_map::DefaultHasher};
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
    let mut shifted: Platform = vec![];

    let len = platform.len();
    for x in 0..len {
        let mut row = vec![];
        let mut empties = vec![];
        let mut rounds = vec![];

        for y in 1..=len {
            let rock = platform[len - y][x];
            match rock {
                Rock::Round => rounds.push(rock),
                Rock::None => empties.push(rock),
                Rock::Cube => {
                    row.append(&mut empties);
                    row.append(&mut rounds);
                    row.push(rock);
                    empties = vec![];
                    rounds = vec![];
                }
            }
        }
        row.append(&mut empties);
        row.append(&mut rounds);
        shifted.push(row);
    }

    *platform = shifted
}

fn count_north_weight(platform: &Platform) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, column)| {
            let mut row_score = 0;

            for &rock in column.iter() {
                match rock {
                    Rock::Round => row_score += idx + 1,
                    _ => {}
                };
            }

            acc + row_score
        })
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

    let mut loop_detector: HashMap<u64, u32> = HashMap::new();

    let mut platform = parse_platform(&document);
    let mut i = 0;
    let mut rotations_left = 1_000_000_000_u32;
    while rotations_left > 0 {
        for _ in 0..4 {
            shift_and_rotate(&mut platform)
        }
        rotations_left -= 1;
        i += 1;

        if let Some(original_idx) = loop_detector.get(&hash_platform(&platform)) {
            rotations_left = (1_000_000_000_u32 - i) % (i - original_idx);
        }
        loop_detector.insert(hash_platform(&platform), i);
    }
    let result = count_north_weight(&platform);

    println!("Result: {}", result);

    println!("Runtime: {:?}", now.elapsed());
}
