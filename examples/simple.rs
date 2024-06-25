use derive_into::ToUsize;
#[derive(ToUsize)]
enum Foo {
    Bar,
    E
}
fn main() {
    let i: usize = Foo::Bar.into();
    let a: usize = Foo::E.into();
    println!("{i}, {a}");
}
