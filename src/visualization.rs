extern crate web_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn stack(_stack: Vec<i64>) {}

#[wasm_bindgen]
pub fn output(output: &str) {
  let window = web_sys::window().unwrap().document().unwrap();
  window
    .query_selector("#output")
    .unwrap()
    .unwrap()
    .set_inner_html(output);
}
