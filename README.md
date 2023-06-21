# ducktor

ducktor is a Rust crate that allows you to create types from `wasm_bindgen::JsValue` using duck typing.

With ducktor, you can define a struct that specifies the fields and types that you expect from a JsValue, and then use
the implement `From<JsValue>` for it using the `#[derive(FromJsValue)]` macro. This way, you can use Rust's type
system and syntax to work with JavaScript objects in WebAssembly.

## Why

- Using`#[wasm_bindgen]` on exported types requires the input on an exported function be of the class created
  by `wasm-bindgen` which is not always possible
- Using imported types with `#[wasm_bindgen] extern "C" { ... }` makes it so that the type is not creatable from Rust.
  This becomes a problem in shared code where you want to create instance of the type from Rust as well.

## Solution

Meet ducktor! A constructor for your structs using duck typing. It allows you to create a struct, as you normally would.
Then, you can use the `#[derive(FromJsValue)]` macro to implement `From<JsValue>` for your struct. This allows you to
use
the `from` method on your struct to create an instance of it from a `JsValue`.

Internally, it defines and imports a non-existent JavaScript type that matches the struct fields and types.
The `JsValue` is then coerced into this type using `JsValue::unchecked_ref` and then the struct is created by getting
the value of each field from the JS type.

## Example

```rust
use ducktor::FromJsValue;

// Define a struct that represents a JavaScript object with an a field and a b field
#[derive(FromJsValue)]
struct Data {
    a: u32,
    b: String,
}

// Create a JavaScript object that conforms to the Data struct
fn roundtrip() {
    let data = js_sys::Object::new();
    js_sys::Reflect::set(&data, &"a".into(), &42.into()).unwrap();
    js_sys::Reflect::set(&data, &"b".into(), &"string".into()).unwrap();

    // Convert the JsValue to a Data using the `from_js_value` method
    let data: Data = Data::from(&data.into());
    assert_eq!(data.a, 42);
    assert_eq!(data.b, "string");
}
```