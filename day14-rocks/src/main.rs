#[derive(Clone,Copy,Debug)]
enum Rock {
    Round,
    Cube,
    None
}

type Platform = Vec<Vec<Rock>>;

fn parse_platform(platform: &str) -> Platform {
    let mut area: Platform = vec![vec![]; platform.split_once("\n").unwrap().0.len()];

    for line in platform.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'o' | 'O' => area[i].push(Rock::Round),
                '#' => area[i].push(Rock::Cube),
                _ => area[i].push(Rock::None)
            }
        }
    }

    area
}

fn count_north_shifted_rounds(platform: Platform) -> usize {
    fn add_rocks(round_count: usize, idx: usize) -> usize {
        (0..round_count).into_iter().fold(0, |acc, i| acc + (idx - i))
    }

    platform.iter().fold(0, |acc, column: &Vec<Rock>| {
        let mut row_score = 0;
        let mut round_count = 0;

        for (idx, &rock) in column.iter().rev().enumerate() {
            match rock {
                Rock::Round => round_count += 1,
                Rock::Cube => {
                    row_score += add_rocks(round_count, idx);
                    round_count = 0
                },
                _ => {}
            };
        };

        row_score += add_rocks(round_count, column.len());

        acc + row_score
    })
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
    
    let platform = parse_platform(&document);
    let result = count_north_shifted_rounds(platform);
    
    println!("Result: {}", result);

    println!("Runtime: {:?}", now.elapsed());
}