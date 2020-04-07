use wasm_bindgen::prelude::*;
mod console;
mod exec;


#[wasm_bindgen(start)]
pub fn run() {
  console::log("Hello, wasm!");
}

#[wasm_bindgen]
pub fn read(input: &str){
  console::log("\n---");
  console::log(&format!("input -> {}", input));

  let code = input.split('\n').collect::<Vec<&str>>();
  let result = exec::run(code);
  console::log(&format!("stack -> {:?}", result));
}

