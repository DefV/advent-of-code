#[derive(Debug)]
pub struct Race {
  duration: i64,
  max_distance: i64
}

impl Race {
  pub fn new(duration: i64, max_distance: i64) -> Self {
    Self {
      duration,
      max_distance
    }
  }

  // Returns all winnable hold-times
  pub fn winnable(&self) -> Vec<i64> {
    (1..self.duration).filter_map(|hold_time| {
      if self.wins_with(hold_time) {
        Some(hold_time)
      } else {
        None
      }
    }).collect()
  }

  pub fn wins_with(&self, hold_time: i64) -> bool {
    let distance_covered = (self.duration - hold_time) * hold_time;
    distance_covered > self.max_distance
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_race_wins_with() {
    let race = Race::new(7, 10);
    assert!(race.wins_with(3));
    assert!(race.wins_with(4));
    assert!(!race.wins_with(5))
  }

  #[test]
  fn test_winnable() {
    let race = Race::new(7, 9);
    assert_eq!(race.winnable(), vec![2, 3, 4, 5])
  }
}