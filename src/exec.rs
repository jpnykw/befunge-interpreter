const TRY_MAX: i16 = std::i16::MAX;
use super::console;
use rand::Rng;

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
          '?' => {
            let direction_id = rand::thread_rng().gen_range(0,4);
            match direction_id {
                0 => direction = (1, 0),
                1 => direction = (-1, 0),
                2 => direction = (0, -1),
                3 => direction = (0, 1),
                _ => {}
            };
          },
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
          ',' => console::log(&format!("{}",stack.pop() as u8 as char)),
          '#' => {
            pointer.0 += direction.0 as usize;
            pointer.1 += direction.1 as usize;
          },
          'p' => {
            let sec : usize = stack.pop() as u8 as usize;
            let fir : usize = stack.pop() as u8 as usize;
            let chr : char = stack.pop() as u8 as char;
            console::log(&format!("sec -> {}",sec));
            console::log(&format!("fir -> {}",fir));
            console::log(&format!("chr -> {}",chr));
            code[sec][fir] = chr;
          },
          'g' => {
            let sec : usize = stack.pop() as u8 as usize;
            let fir : usize = stack.pop() as u8 as usize;
            stack.push(code[sec][fir] as i64);
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
    console::log(&format!("pointer: {:?}", pointer));
    try_count += 1;
  }
  console::log(&format!("end: {:?}",code));

  stack.data
}