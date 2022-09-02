use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn say_hello();

    pub fn say_hello_name(name: &str);

    pub fn add_bang(string: &str) -> String;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_number_list(numbers: Vec<f64>);

    pub fn list_it(a: f64, b: f64) -> Vec<f64>;

    pub fn log_each(numbers: Vec<f64>);

    #[wasm_bindgen(js_namespace = _, js_name = concat)]
    pub fn lodash_concat(a: &[f64], b: &[f64], c: &[f64]) -> Vec<f64>;
}
