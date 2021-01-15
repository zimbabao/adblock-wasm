use wasm_bindgen::prelude::*;
use adblock::engine::Engine;
use adblock::lists::{FilterFormat, FilterSet};
use once_cell::sync::Lazy;

// TODO:
// 1. Figure out how to manage context and object and share
// 2. Reduce unsafe blocks. Current it works fine because JS is single threaded
//    we are expected to see only serialized call from browser.
static mut engine: Lazy<Vec<Engine>> = Lazy::new(|| {
    let m = vec![];
    m
});

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn generate_filter_set(rules_string: &str) -> FilterSet {
    let rules = rules_string.split("\n");

    let mut rules_vec = vec![];
    for rule in rules {
        rules_vec.push(String::from(rule))
    }
    let mut filter_set = FilterSet::new(true);
    filter_set.add_filters(&rules_vec, FilterFormat::Standard);

    return filter_set;
}

fn create_new_engine(rules_string: &str) -> Engine {
    return Engine::from_filter_set(generate_filter_set(rules_string), true)
}

#[wasm_bindgen]
pub fn init(rules_string: &str) {
    unsafe {
        let e = create_new_engine(rules_string);
        engine.clear();
        engine.push(e);
    }
}

#[wasm_bindgen]
pub fn check(url: &str, init: &str, req_type: &str) -> bool {
    unsafe {
        if engine.len() == 0 {
            return false;
        }
        for i in 0..engine.len() {
            let result = engine[i].check_network_urls(url, init, req_type);
            if result.matched {
                return true;
            }
            return result.matched;
        }
        return false;
    }
}

#[wasm_bindgen]
pub fn add_rules(rules_string: &str) -> bool {
    unsafe {
        engine.push(create_new_engine(rules_string));
    }
    return true;
}