## Derive macro to get the variant name of an enum

### Documentation

The documentation for this crate can be found [here](https://bowarc.github.io/crates/enum_variant_name)

#### Use example:

cargo.toml
```toml
[dependencies]
enum_variant_name = {git = "https://github.com/Bowarc/Crates.git", package = "enum_variant_name"}
``` 
main.rs
```rust
#[derive(enum_variant_name::VariantName)]
enum MyEnum{
  Variant1,
  Variant2(String),
  Variant3{
    field1: String,
  },
}

let my_enum = MyEnum::Variant1;
println!("{}", my_enum.variant_name()); // Variant1

let my_enum = MyEnum::Variant2(String::from("Hi"));
println!("{}", my_enum.variant_name()); // Variant2

let my_enum = MyEnum::Variant3{field1: String::from("Hellow")};
println!("{}", my_enum.variant_name()); // Variant3
``` 
