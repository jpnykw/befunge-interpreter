extern crate web_sys;
use wasm_bindgen::prelude::*;
use super::console;

#[wasm_bindgen]
pub fn stack(stack:Vec<i64>){
  //
}

#[wasm_bindgen]
pub fn output(output:&str){
  let window = web_sys::window().unwrap().document().unwrap();
  window.query_selector("#Output").unwrap().unwrap().set_inner_html(output);
}