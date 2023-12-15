#[derive(Debug)]
enum Modifier {
  Add,
  Remove,
}

#[derive(Debug)]
struct Operation {
  key: String,
  modifier: Modifier,
  power: u16
}

impl From<&str> for Operation {
  fn from(command: &str) -> Self {
    let modifier_idx = command.find(|c| c == '=' || c == '-').unwrap();
    let key = command[..modifier_idx].to_string();
    let modifier = &command[modifier_idx..=modifier_idx];
    let power = &command[modifier_idx + 1..];

    let modifier = match modifier {
      "=" => Modifier::Add,
      "-" => Modifier::Remove,
      _ => unreachable!()
    };

    Self {
      key,
      modifier,
      power: power.parse().unwrap_or(0)
    }
  }
}

impl Operation {
  fn idx(&self) -> usize {
    hash(&self.key) as usize
  }
}

#[derive(Debug)]
struct Box {
  key: String,
  power: u16
}

#[derive(Debug)]
pub struct Boxes {
  boxes: [Vec<Box>; 256]
}

impl Boxes {
  pub fn new() -> Self {
    let boxes: [Vec<Box>; 256] = array_init::array_init(|_| Vec::new());

    Self {
      boxes
    }
  }

  pub fn process_command(&mut self, command: &str) {
    let op: Operation = command.into();
    
    let bx = &mut self.boxes[op.idx()];

    match op.modifier {
      Modifier::Add => {
        if let Some(box_idx) = bx.iter().position(|b| b.key == op.key) {
          bx[box_idx].power = op.power;
        } else {
          bx.push(Box {
            key: op.key,
            power: op.power
          });
        }
      },
      Modifier::Remove => {
        if let Some(box_idx) = bx.iter().position(|b| b.key == op.key) {
          bx.remove(box_idx);
        }
      }
    }
  }

  pub fn total_power(&self) -> u64 {
    self.boxes.iter().enumerate().flat_map(|(idx, slots)| {
      slots.iter().enumerate().map(move |(slot_idx, slot)| {
        (idx + 1) as u64 * (slot_idx + 1) as u64 * slot.power as u64
      })
    }).sum()
  }
}

pub fn hash(string: &str) -> u8 {
  string.chars().fold(0, |hash, c| 
    hash.overflowing_add(c as u8).0.overflowing_mul(17).0
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("rn"), 0);
    assert_eq!(hash("cm"), 0);
  }
}