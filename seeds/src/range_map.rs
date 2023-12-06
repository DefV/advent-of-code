use std::ops::Range;
#[derive(Debug)]
struct PointedRange {
    source_range: Range<u64>,
    destination_offset: i64,
}

pub struct RangeMap {
    ranges: Vec<PointedRange>,
}

impl RangeMap {
    pub fn from_str(map_str: &str) -> Self {
        let mut ranges: Vec<PointedRange> = map_str
            .lines()
            .map(|line| {
                let parts: Vec<u64> = line
                    .split_whitespace()
                    .map(str::parse::<u64>)
                    .flat_map(Result::ok)
                    .collect();

                let source_range: Range<u64> = Range {
                    start: parts[1],
                    end: parts[1] + parts[2],
                };
                let destination_offset: i64 = parts[0] as i64 - parts[1] as i64;

                PointedRange {
                    source_range,
                    destination_offset,
                }
            })
            .collect();

        ranges.sort_by_key(|range| range.source_range.start);

        Self { ranges }
    }

    pub fn get(&self, source: u64) -> u64 {
      let ranges = &self.ranges;
        ranges.into_iter()
            .find(|&range| range.source_range.contains(&source))
            .and_then(|range| source.checked_add_signed(range.destination_offset))
            .unwrap_or(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rangemap_from_str() {
        let input = "50 98 2\n52 50 48";
        let map = RangeMap::from_str(input);

        assert_eq!(map.ranges.len(), 2);
        assert_eq!(map.ranges[0].destination_offset, 2);
        assert_eq!(map.ranges[0].source_range.start, 50);
        assert_eq!(map.ranges[0].source_range.end, 98);
    }

    #[test]
    fn test_rangemap_get() {
        let input = "50 98 2\n52 50 48";
        let map = RangeMap::from_str(input);

        assert_eq!(map.get(14), 14);
        assert_eq!(map.get(79), 81);
        assert_eq!(map.get(97), 99);
        assert_eq!(map.get(99), 51);
    }
}
