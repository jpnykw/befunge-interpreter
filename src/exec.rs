const TRY_MAX: i16 = std::i16::MAX;
use super::console;

pub fn run (
  code: Vec<&str>
) -> Vec<i64> {
  let mut direction: (i32, i32) = (1, 0); // (dx, dy)
  let mut pointer: (usize, usize) = (0, 0); // (x, y)
  let mut stack: Vec<i64> = Vec::new();
  let mut try_count = 0;

  loop {
    match try_count {
      TRY_MAX => return vec![2i64], // over

      _ => {
        let line = code[pointer.1];
        if line.len() <= pointer.0 {
          return vec![2i64];
        }

        // https://en.wikipedia.org/wiki/Befunge
        // Befunge-93 instruction list
        let instruct = line.chars().nth(pointer.0).unwrap();
        match instruct {
          '0' ... '9' => stack.push(instruct as i64 - 48),
          '+' => {},
          '-' => {},
          '*' => {},
          '/' => {},
          '%' => {},
          '!' => {},
          '`' => {},
          '>' => direction = (1, 0),
          '<' => direction = (-1, 0),
          '^' => direction = (0, -1),
          'v' => direction = (0, 1),
          '?' => {},
          '_' => {},
          '|' => {},
          '"' => {},
          ':' => {},
          '\\' => {},
          '$' => {},
          '.' => console::log(&format!("{:?} ", stack.pop().unwrap())),
          ',' => {},
          '#' => {},
          'p' => {},
          'g' => {},
          '&' => {},
          '~' => {},
          '@' => break,
          _ => {}
        };
      }
    };

    pointer.0 += direction.0 as usize;
    pointer.1 += direction.1 as usize;
    try_count += 1;
  }

  stack
}

