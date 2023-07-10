#[derive(::ducktor::FromJsValue)]
pub struct Data {
    a: u32,
    b: i64,
}

#[::wasm_bindgen::prelude::wasm_bindgen]
pub fn test(data: ::wasm_bindgen::JsValue) {
    let data: Data = Data::from(&data);
    let _ = data.a;
    let _ = data.b;
}

pub fn test_2(data: Data) -> wasm_bindgen::JsValue {
    let data = Data {
        a: 0,
        b: 0,
    };

    data.into()

}

fn main() {}