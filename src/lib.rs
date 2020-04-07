use wasm_bindgen::prelude::*;
mod exec;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace=console)]
  fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
  log("Hello, wasm!");
}

#[wasm_bindgen]
pub fn read(input: &str){
  log("\n---");
  log(&format!("input -> {}", input));

  let code = input.split('\n').collect::<Vec<&str>>();
  let result = exec::run(code);
  log(&format!("stack -> {:?}", result));
}

