use ducktor::FromJsValue;

#[derive(FromJsValue)]
enum Data {}

#[derive(FromJsValue)]
union Union {

}

#[derive(FromJsValue)]
struct UnitStruct;

#[derive(FromJsValue)]
struct TupleStruct(u32, i64);

fn main() {}