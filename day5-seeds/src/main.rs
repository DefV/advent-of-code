use std::collections::HashMap;
use rayon::prelude::*;

pub mod range_map;
use range_map::RangeMap;

fn main() {
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

    let (seeds, maps) = parse_document(document.as_str());
    let min = parallel_find_min(seeds, maps);

    println!("Min: {}", min);
}

struct Seeds {
    seed_rules: Vec<(u64, u64)>,
}
struct SeedGenerator<'a> {
    seeds: &'a Seeds,
    rule_index: usize,
    last: u64,
}


impl<'a> Iterator for SeedGenerator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let seed_rule = self.seeds.seed_rules[self.rule_index];
        let next = seed_rule.0 + self.last;

        if self.last >= seed_rule.1 {
            if self.rule_index >= self.seeds.seed_rules.len() - 1 {
                return None;
            }

            self.rule_index += 1;
            self.last = 0;
            println!("Incrementing rule index ({} / {})", self.rule_index + 1, self.seeds.seed_rules.len());
        } else {
            self.last += 1;
        }
        Some(next)
    }
}

impl Seeds {
    pub fn iter(&self) -> SeedGenerator {
        SeedGenerator {
            seeds: self,
            rule_index: 0,
            last: 0,
        }
    }
}

fn parallel_find_min(seeds: Seeds, maps: HashMap<&str, RangeMap>) -> u64 {
    seeds.iter()
        // .par_bridge()
        .map(|i| resolve_seed_location(i, &maps))
        .min()
        .unwrap()
}

fn resolve_seed_location(seed: u64, maps: &HashMap<&str, RangeMap>) -> u64 {
    let mut i = seed;
    i = maps.get("seed-to-soil").unwrap().get(i);
    i = maps.get("soil-to-fertilizer").unwrap().get(i);
    i = maps.get("fertilizer-to-water").unwrap().get(i);
    i = maps.get("water-to-light").unwrap().get(i);
    i = maps.get("light-to-temperature").unwrap().get(i);
    i = maps.get("temperature-to-humidity").unwrap().get(i);
    i = maps.get("humidity-to-location").unwrap().get(i);
    i
}

fn parse_document(document: &str) -> (Seeds, HashMap<&str, RangeMap>) {
    let mut seeds: Seeds = Seeds { seed_rules: vec![] };
    let mut maps: HashMap<&str, RangeMap> = HashMap::new();

    let mut map_buffer = String::new();
    let mut map_name = "";

    for line in document.lines() {
        if line.contains("seeds: ") {
            seeds = generate_seeds(line.trim_start_matches("seeds: "));
        } else if line.contains("map:") {
            map_name = line.split_whitespace().next().unwrap()
        } else if line == "" {
            if map_buffer.len() > 0 {
                maps.insert(map_name, RangeMap::from_str(map_buffer.as_str()));
                map_buffer = String::new();
            }
        } else {
            map_buffer.push_str(line);
            map_buffer.push_str("\n");
        }
    }

    (seeds, maps)
}

fn generate_seeds(seed_str: &str) -> Seeds {
    let mut seed_definitions = seed_str
        .split_whitespace()
        .map(str::parse::<u64>)
        .flat_map(Result::ok);
    let mut seed_rules: Vec<(u64, u64)> = vec![];

    while let Some(start) = seed_definitions.next() {
        let Some(count) = seed_definitions.next() else {
            panic!("Nope")
        };

        seed_rules.push((start, count));
    }

    Seeds { seed_rules }
}



#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_parse_document() {
        let input = "seeds: 79 14 55 13

test-one map:
50 98 2
52 50 48

test-two map:
60 56 37
56 93 4

";

        let (_seeds, maps) = parse_document(input);

        assert_eq!(maps.len(), 2);
        assert_eq!(maps.get("test-one").unwrap().get(14), 14);
    }
}
