const TRY_MAX: i8 = std::i8::MAX;

pub fn run (
  code: &Vec<char>
) -> Vec<i64> {
  let mut pointer: (i32, i32) = (0, 0);
  let mut direction: (i32, i32) = (1, 0);

  let mut try_count = 0;
  loop {
    pointer.0 += direction.0;
    pointer.1 += direction.1;
    try_count += 1;

    if try_count > TRY_MAX {
      break;
    }
  }

  if try_count > TRY_MAX {
    vec![2i64]
  } else {
    vec![1i64]
  }
}

