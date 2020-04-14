use wasm_bindgen::prelude::*;
mod console;
mod exec;

#[wasm_bindgen(start)]
pub fn run() {
  console::log("Hello, wasm!");
}

#[wasm_bindgen]
pub fn read(input: &str){
  let code = input.split('\n').collect::<Vec<&str>>();

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

  console::log("\n---");
  console::log(&format!("input -> {}", input));

  let result = exec::run(frame);
  console::log(&format!("stack -> {:?}", result));
}

