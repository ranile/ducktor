use ducktor::FromJsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(test_in_browser)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(FromJsValue)]
struct Data {
    a: u32,
    b: String,
}

#[wasm_bindgen_test]
fn roundtrip() {
    let data = js_sys::Object::new();
    js_sys::Reflect::set(&data, &"a".into(), &42.into()).unwrap();
    js_sys::Reflect::set(&data, &"b".into(), &"string".into()).unwrap();

    let data: Data = Data::from_js_value(&data.into());
    assert_eq!(data.a, 42);
    assert_eq!(data.b, "string");
}
