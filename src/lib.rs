use wasm_bindgen::prelude::*;
mod console;
mod exec;


#[wasm_bindgen(start)]
pub fn run() {
  console::log("Hello, wasm!");
}

#[wasm_bindgen]
pub fn read(input: &str){
  if input.chars().count() as f32 > (std::i8::MAX as f32).powf(2f32) {
    console::log("too long");
    return;
  }

  console::log("\n---");
  console::log(&format!("input -> {}", input));

  let code = input.split('\n').collect::<Vec<&str>>();
  let result = exec::run(code);
  console::log(&format!("stack -> {:?}", result));
}

