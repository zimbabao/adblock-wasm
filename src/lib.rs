use wasm_bindgen::prelude::*;
use adblock;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn new(url: &str) {
    alert(&format!("Hello, {}!", url));
}

#[wasm_bindgen]
pub fn check(name: &str) -> bool {
    return true;
}

// #[wasm_bindgen]
// pub fn clean(name: &str) -> boolean {
//     return true;
// }