#[derive(::ducktor::FromJsValue)]
struct Data {
    a: u32,
    b: i64,
}

#[::wasm_bindgen::prelude::wasm_bindgen]
pub fn test(data: ::wasm_bindgen::JsValue) {
    let data: Data = Data::from(&data);
    let _ = data.a;
    let _ = data.b;
}

fn main() {}