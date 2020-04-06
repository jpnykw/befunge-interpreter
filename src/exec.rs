const TRY_MAX: i16 = std::i16::MAX;

pub fn run (
  input: &str
) -> Vec<i64> {
  let temp = input.split('\n').collect::<Vec<&str>>();
  let mut code = temp.iter().map(|line| line.chars().collect::<Vec<char>>());
  let mut direction: (usize, usize) = (1, 0); // (dx, dy)
  let mut pointer: (usize, usize) = (0, 0); // (x, y)
  let mut try_count = 0;

  loop {
    match try_count {
      TRY_MAX => return vec![2i64], // over
      _ => {
        match code.nth(pointer.1) {
          Some(line) => {
            if line.len() <= pointer.0 { return vec![2i64]; }
            match line[pointer.0] {
              '@' => break,
              _ => {}
            };
          },
          None => return vec![2i64]
        };
      }
    };

    pointer.0 += direction.0;
    pointer.1 += direction.1;
    try_count += 1;
  }

  vec![1i64] // finish
}

