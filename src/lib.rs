use wasm_bindgen::prelude::*;
mod exec;
mod log;


#[wasm_bindgen(start)]
pub fn run() {
  log::log("Hello, wasm!");
}

#[wasm_bindgen]
pub fn read(input: &str){
  log::log("\n---");
  log::log(&format!("input -> {}", input));

  let code = input.split('\n').collect::<Vec<&str>>();
  let result = exec::run(code);
  log::log(&format!("stack -> {:?}", result));
}

