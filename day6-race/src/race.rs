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

  pub fn winnable_binary(&self) -> i64 {
    let mut lower_bound = 0;
    let mut upper_bound = self.duration / 2;

    while upper_bound - lower_bound > 1 {
      let pos = (lower_bound + upper_bound) / 2;
      if self.wins_with(pos) {
        upper_bound = pos;
      } else {
        lower_bound = pos;
      }
    }
    
    return self.duration - (lower_bound * 2 + 1);
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

  #[test]
  fn test_winnable_binary() {
    let race = Race::new(38, 241);
    assert_eq!(race.winnable_binary(), race.winnable().len() as i64);

    let race2 = Race::new(94, 1549);
    assert_eq!(race2.winnable_binary(), race2.winnable().len() as i64);
  }
}