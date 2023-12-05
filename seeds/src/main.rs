use std::collections::HashMap;

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
    let min = seeds.iter().map(|i| maps.get("seed-to-soil").unwrap().get(*i) )
                .map(|i| maps.get("soil-to-fertilizer").unwrap().get(i) )
                .map(|i| maps.get("fertilizer-to-water").unwrap().get(i) )
                .map(|i| maps.get("water-to-light").unwrap().get(i) )
                .map(|i| maps.get("light-to-temperature").unwrap().get(i) )
                .map(|i| maps.get("temperature-to-humidity").unwrap().get(i) )
                .map(|i| maps.get("humidity-to-location").unwrap().get(i) )
                .min().unwrap();

    println!("Min: {}", min);
}

struct PointedRange {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}
struct RangeMap {
    ranges: Vec<PointedRange>,
}

impl RangeMap {
    fn get(&self, source: u64) -> u64 {
        let mut destination = source;

        for range in &self.ranges {
            if source >= range.source_range_start
                && source < range.source_range_start + range.range_length
            {
                destination = range.destination_range_start + (source - range.source_range_start);
                break;
            }
        }

        destination
    }
}

fn parse_document(document: &str) -> (Vec<u64>, HashMap<&str, RangeMap>) {
    let mut seeds: Vec<u64> = vec![];
    let mut maps: HashMap<&str, RangeMap> = HashMap::new();

    let mut map_buffer = String::new();
    let mut map_name = "";

    for line in document.lines() {
        if line.contains("seeds: ") {
            seeds = line
                .split_whitespace()
                .skip(1)
                .map(str::parse::<u64>)
                .flat_map(Result::ok)
                .collect();
        } else if line.contains("map:") {
            map_name = line.split_whitespace().next().unwrap()
        } else if line == "" {
            if map_buffer.len() > 0 {
                maps.insert(map_name, generate_map(map_buffer.as_str()));
                map_buffer = String::new();
            }
        } else {
            map_buffer.push_str(line);
            map_buffer.push_str("\n");
        }
    }

    (seeds, maps)
}

fn generate_map(map_str: &str) -> RangeMap {
    let mut ranges: Vec<PointedRange> = map_str
        .lines()
        .map(|line| {
            let parts: Vec<u64> = line
                .split_whitespace()
                .map(str::parse::<u64>)
                .flat_map(Result::ok)
                .collect();

            PointedRange {
                destination_range_start: parts[0],
                source_range_start: parts[1],
                range_length: parts[2],
            }
        })
        .collect();

    ranges.sort_by_key(|range| range.source_range_start);

    RangeMap { ranges }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_map() {
        let input = "50 98 2\n52 50 48";
        let map = generate_map(input);

        assert_eq!(map.ranges.len(), 2);
        assert_eq!(map.ranges[0].source_range_start, 50);
        assert_eq!(map.ranges[0].destination_range_start, 52);
        assert_eq!(map.ranges[0].range_length, 48);
    }

    #[test]
    fn test_rangemap_get() {
        let input = "50 98 2\n52 50 48";
        let map = generate_map(input);

        assert_eq!(map.get(14), 14);
        assert_eq!(map.get(79), 81);
        assert_eq!(map.get(97), 99);
        assert_eq!(map.get(99), 51);
    }

    #[test]
    fn test_parse_document() {
        let input = "seeds: 79 14 55

test-one map:
50 98 2
52 50 48

test-two map:
60 56 37
56 93 4

";

        let (seeds, maps) = parse_document(input);

        assert_eq!(seeds.len(), 3);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);

        assert_eq!(maps.len(), 2);
        assert_eq!(maps.get("test-one").unwrap().get(14), 14);
    }
}
