use core::panic;

pub type Sequence = u64;

#[derive(Debug, PartialEq)]
pub enum Reflection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Pattern {
    rows: Vec<Sequence>,
    columns: Vec<Sequence>,
}

impl From<&str> for Pattern {
    fn from(document: &str) -> Self {
        let column_count = document.lines().next().unwrap().len();
        let mut column_chars: Vec<Vec<char>> = vec![vec![]; column_count];

        let rows = Box::new(document.lines().map(str::chars))
            .inspect(|chars| {
                chars
                    .clone()
                    .enumerate()
                    .for_each(|(idx, c)| column_chars[idx].push(c))
            })
            .map(|chars| parse_sequence(chars))
            .collect();

        let columns = column_chars
            .into_iter()
            .map(|chars| parse_sequence(chars.into_iter()))
            .collect();

        Self { rows, columns }
    }
}

fn parse_sequence<I>(iter: I) -> Sequence
where
    I: IntoIterator<Item = char>,
{
    let mut sequence = 0;

    for c in iter {
        let bit = match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("Not a known character"),
        };

        sequence = (sequence << 1) | bit;
    }

    sequence
}

// This logic needs some love, but it works for now
fn find_reflection(sequences: &Vec<Sequence>) -> Option<usize> {
    let mut sequences = sequences.clone();

    while sequences.len() > 1 {
        if sequences.iter().rev().copied().collect::<Vec<u64>>() == sequences {
            return Some(sequences.len() / 2);
        }

        sequences.pop();
    }

    None
}

fn smudge_reflection_idx(sequences: &Vec<Sequence>) -> Option<usize> {
    (0..sequences.len() - 1).find_map(|idx| {
        if (0..idx + 1)
            .into_iter()
            .rev()
            .zip(idx + 1..sequences.len())
            .fold(0, |acc, (i, j)| {
                acc + (sequences[i] ^ sequences[j]).count_ones()
            })
            == 1
        {
            Some(idx + 1)
        } else {
            None
        }
    })
}

impl Pattern {
    pub fn find_smudge_reflection(&self) -> (usize, Reflection) {
        if let Some(reflection) = smudge_reflection_idx(&self.rows) {
            (reflection, Reflection::Horizontal)
        } else if let Some(reflection) = smudge_reflection_idx(&self.columns) {
            (reflection, Reflection::Vertical)
        } else {
            unreachable!()
        }
    }

    pub fn find_reflection(&self) -> (usize, Reflection) {
        if let Some(reflection) = find_reflection(&self.rows) {
            (reflection, Reflection::Horizontal)
        } else if let Some(reflection) = find_reflection(&self.columns) {
            (reflection, Reflection::Vertical)
        } else if let Some(reflection) = find_reflection(&self.rows.iter().rev().copied().collect())
        {
            (self.rows.len() - reflection, Reflection::Horizontal)
        } else if let Some(reflection) =
            find_reflection(&self.columns.iter().rev().copied().collect())
        {
            (self.columns.len() - reflection, Reflection::Vertical)
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_PATTERN: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const SECOND_PATTERN: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parse_sequence() {
        let sequence = parse_sequence(vec!['.', '#', '.', '#'].into_iter());
        assert_eq!(sequence, 0b0101);
    }

    #[test]
    fn test_pattern_from_str() {
        let pattern = Pattern::from(FIRST_PATTERN);
        assert_eq!(pattern.rows.len(), 7);
        assert_eq!(pattern.columns.len(), 9);

        assert_eq!(pattern.rows[0], 0b101100110);
        assert_eq!(pattern.columns[0], 0b1011001);
    }

    #[test]
    fn test_find_middle() {
        let pattern = Pattern::from(FIRST_PATTERN);
        assert_eq!(pattern.find_reflection(), (5, Reflection::Vertical));

        let pattern = Pattern::from(SECOND_PATTERN);
        assert_eq!(pattern.find_reflection(), (4, Reflection::Horizontal));
    }

    #[test]
    fn test_find_smudge_middle() {
        let pattern = Pattern::from(FIRST_PATTERN);
        assert_eq!(
            pattern.find_smudge_reflection(),
            (3, Reflection::Horizontal)
        );

        let pattern = Pattern::from(SECOND_PATTERN);
        assert_eq!(pattern.find_smudge_reflection(), (1, Reflection::Horizontal));
    }
}
