use js_sys::Math;
use wasm_bindgen::prelude::*;

const LINKS_LIST: &str = include_str!("../links.txt");

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let list: Vec<&str> = LINKS_LIST.split_terminator('\n').collect();
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");

    let idx = (Math::random() * list.len() as f64) as usize;

    window.location().assign(list[idx]).unwrap();

    Ok(())
}
