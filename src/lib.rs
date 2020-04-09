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

  if code.len() > 127 {
    console::log("too long");
    return;
  }
  for i in &code {
    if i.len() > 127 {
      console::log("too long");
      return;
    }
  }

  console::log("\n---");
  console::log(&format!("input -> {}", input));

  let result = exec::run(code);
  console::log(&format!("stack -> {:?}", result));
}

