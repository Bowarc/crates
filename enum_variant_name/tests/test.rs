use std::assert_eq;

#[test]
fn usage(){
    #[derive(enum_variant_name::VariantName)]
    enum MyEnum{
      Variant1,
      Variant2(String),
      Variant3{
        field1: String,
      },
    }

    assert_eq!(MyEnum::Variant1.variant_name(), "Variant1");
    assert_eq!(MyEnum::Variant2(String::from("Hi")).variant_name(), "Variant2");
    assert_eq!(MyEnum::Variant3{field1: String::from("Hellow")}.variant_name(), "Variant3");
}
