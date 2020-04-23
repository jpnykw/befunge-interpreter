extern crate web_sys;
use wasm_bindgen::prelude::*;
use super::console;

#[wasm_bindgen]
pub fn isStepMode() -> bool {
  let window = web_sys::window().unwrap().document().unwrap();
  let checkbox = window.query_selector("#mode").unwrap().unwrap();

  checkbox.get_attribute("value").unwrap() == "true"
}
