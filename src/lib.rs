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
pub fn read(code: &str){
  log(code);
  let temp = code.chars().collect::<Vec<char>>();
  log(&format!("{:?}, {:?}", &temp, exec::run(&temp)));
}

