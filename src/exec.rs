use std::char;
use std::{thread, time};
use std::time::Duration;

use super::console;
use rand::Rng;
use super::visualization;

const TRY_MAX: i16 = std::i16::MAX;

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
  mut code: Vec<Vec<char>>,
  input: &str,
  mode: bool
) -> Vec<i64> {
  let mut direction: (i32, i32) = (1, 0);
  let mut pointer: (usize, usize) = (0, 0);

  let mut output = String::new();
  let mut stack = Stack { data: Vec::new() };

  let mut try_count = 0;
  let mut letter_input = input.chars();
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

          // '.' => console::log(&format!("{:?} ", stack.pop())),
          '.' => output = format!("{}{}", output, stack.pop()),

          // ',' => console::log(&format!("{}",char::from_u32(stack.pop() as u32).unwrap())),
          ',' => output = format!("{}{}", output, char::from_u32(stack.pop() as u32).unwrap()),

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

          '&' => {
            let mut push_data : i64 = 0;
            let mut synbol: bool = false;
            let mut input_nth = letter_input.nth(0);

            if input_nth == Some('-') {
              input_nth = letter_input.nth(0);
              synbol = true;
            }

            while input_nth != None {
              let input_char = input_nth.unwrap();
              if '0' <= input_char && input_char <= '9' {
                push_data = push_data*10+ ((input_char as u8)-'0' as u8) as i64;
                input_nth = letter_input.nth(0);
              }else{
                break;
              }
            }

            push_data += if synbol { -1 } else { 0 };
            stack.push(push_data);
          },

          '~' => {
            match letter_input.nth(0) {
              Some(v) => stack.push(v as i64),
              None => {}
            };
          },

          '@' => break,

          _ => {}
        };
      }
    };

    pointer.0 += direction.0 as usize;
    pointer.1 += direction.1 as usize;
    pointer.0 %= 128;
    pointer.1 %= 128;

    visualization::output(&output);
    try_count += 1;

    if mode {
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
  }

  console::log("output:");
  console::log(&format!("  {}", output));
  stack.data
}
