fn main() {}

#[derive(derive_macro_example::MyDebug)]
struct Foo<T> {
    a: u32,
    b: T,
    c: u32,
}
