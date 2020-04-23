use wasm_bindgen::prelude::*;

mod visualization;
mod option;

mod console;
mod exec;

#[wasm_bindgen]
pub fn read(input: &str, pre_code: &str){
  let code = pre_code.split('\n').collect::<Vec<&str>>();

  if option::isStepMode() {
    console::log("OK");
  }

  if code.len() > 128 {
    console::log("too long");
    return;
  }

  for i in &code {
    if i.len() > 128 {
      console::log("too long");
      return;
    }
  }

  let mut frame: Vec<Vec<char>> = vec![vec![' '; 128]; 128];
  for i in 0..code.len() {
    let mut chs = code[i].chars();
    for j in 0..code[i].chars().count(){
      frame[i][j] = chs.next().unwrap();
    }
  }

  console::log("");

  console::log("code:");
  console::log(&format!("  {}", pre_code));

  if input.chars().count() > 0 {
    console::log("input:");
    console::log(&format!("  {}", input));
  }

  console::log("");

  // execute
  let stack = exec::run(frame, input);

  console::log("");
  console::log("stack:");
  console::log(&format!("{:?}", stack));
}

