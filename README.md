Derive for From and Into usize.
```rust
use derive_into::ToUsize;
#[derive(ToUsize)]
enum Foo {
    Bar,
    E
}
assert_eq!(0_usize, Foo::Bar.into());
assert_eq!(1_usize, Foo::E.into());
```
Same as:
```rust
enum Foo {
    Bar,
    E
}
impl From<Foo> for usize {
    fn from(value: Foo) -> usize {
        Foo as usize
    }
}
```
