const TRY_MAX: i16 = std::i16::MAX;
use super::console;

struct Stack {
  data: Vec<i64>
}

impl Stack {
  fn push(&mut self, value: i64) {
    self.data.push(value);
  }

  fn pop(&mut self) -> i64 {
    if self.data.len() == 0 { 0 }
    else { self.data.pop().unwrap() }
  }
}


pub fn run (
  code: Vec<Vec<char>>
) -> Vec<i64> {
  let mut direction: (i32, i32) = (1, 0);
  let mut pointer: (usize, usize) = (0, 0);
  let mut stack = Stack { data: Vec::new() };
  let mut try_count = 0;

  loop {
    match try_count {
      TRY_MAX => return vec![2i64], // over

      _ => {
        let line = &code[pointer.1];
        if line.len() <= pointer.0 {
          return vec![2i64];
        }

        // https://en.wikipedia.org/wiki/Befunge
        // Befunge-93 instruction list
        let instruct = line[pointer.0];
        match instruct {
          '0' ... '9' => stack.push(instruct as i64 - 48),
          '+' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(fir+sec);
          },
          '-' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(fir-sec);
          },
          '*' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(fir*sec);
          },
          '/' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(fir/sec);
          },
          '%' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(fir%sec);
          },
          '!' => {
            let fir = stack.pop();
            if fir == 0 {
              stack.push(1);
            } else {
              stack.push(0);
            }
          },
          '`' => {
            let sec = stack.pop();
            let fir = stack.pop();
            if fir > sec {
              stack.push(1);
            } else {
              stack.push(0);
            }
          },
          '>' => direction = (1, 0),
          '<' => direction = (-1, 0),
          '^' => direction = (0, -1),
          'v' => direction = (0, 1),
          '?' => {},
          '_' => direction = if stack.pop() == 0 { (1, 0) } else { (-1, 0) },
          '|' => direction = if stack.pop() == 0 { (0, 1) } else { (0, -1) },
          '"' => {},
          ':' => {
            let fir = stack.pop();
            stack.push(fir);
            stack.push(fir);
          },
          '\\' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(sec);
            stack.push(fir);
          },
          '$' => {
            stack.pop();
          },
          '.' => console::log(&format!("{:?} ", stack.pop())),
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
    pointer.0 %= 128;
    pointer.1 %= 128;
    console::log(&format!("pointer: {:?}", pointer));
    try_count += 1;
  }

  stack.data
}
