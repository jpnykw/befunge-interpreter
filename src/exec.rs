const TRY_MAX: i16 = std::i16::MAX;
use super::console;
use rand::Rng;
use std::char;

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
  mut code: Vec<Vec<char>>
) -> Vec<i64> {
  let mut direction: (i32, i32) = (1, 0);
  let mut pointer: (usize, usize) = (0, 0);
  let mut stack = Stack { data: Vec::new() };

  let mut try_count = 0;
  let mut double_quotation_flag = false;

  loop {
    match try_count {
      TRY_MAX => return vec![2i64],

      _ => {
        let line = &code[pointer.1];
        if line.len() <= pointer.0 {
          return vec![2i64];
        }

        let mut instruct = line[pointer.0];

        if double_quotation_flag {
          match instruct {
            '"' => double_quotation_flag = false,
            _ => stack.push(instruct as i64)
          };

          instruct = ' ';
        }

        match instruct {
          '0' ..= '9' => stack.push(instruct as i64 - 48),

          '+' | '-' | '*' | '/' | '%' => {
            let sec = stack.pop();
            let fir = stack.pop();
            stack.push(
              match instruct {
                '+' => fir + sec,
                '-' => fir - sec,
                '*' => fir * sec,
                '/' => fir / sec,
                '%' => fir % sec,
                _ => 0
              }
            );
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

          '>' | '<' | '^' | 'v' => {
            direction = match instruct {
              '>' => (1, 0),
              '<' => (-1, 0),
              '^' => (0, -1),
              'v' => (0, 1),
              _ => (0, 0)
            };
          }

          '?' => {
            let direction_id = rand::thread_rng().gen_range(0,4);
            direction = match direction_id {
                0 => (1, 0),
                1 => (-1, 0),
                2 => (0, -1),
                3 => (0, 1),
                _ => (0, 0)
            };
          },

          '_' => direction = if stack.pop() == 0 { (1, 0) } else { (-1, 0) },

          '|' => direction = if stack.pop() == 0 { (0, 1) } else { (0, -1) },

          '"' => double_quotation_flag = true,

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

          ',' => console::log(&format!("{}",char::from_u32(stack.pop() as u32).unwrap())),

          '#' => {
            pointer.0 += direction.0 as usize;
            pointer.1 += direction.1 as usize;
          },

          'p' => {
            let sec : usize = stack.pop() as usize;
            let fir : usize = stack.pop() as usize;
            let chr : char = stack.pop() as u8 as char;
            code[sec%128][fir%128] = chr;
          },

          'g' => {
            let sec : usize = stack.pop() as usize;
            let fir : usize = stack.pop() as usize;
            stack.push(code[sec%128][fir%128] as i64);
          },

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
    try_count += 1;
  }

  stack.data
}
