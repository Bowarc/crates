use std::assert_eq;

#[test]
fn usage(){
    #[derive(enum_variant_name::VariantName)]
    enum MyEnum{
      Variant1,
      Variant2(()),
      Variant3{
        _field1: (),
      },
    }

    assert_eq!(MyEnum::Variant1.variant_name(), "Variant1");
    assert_eq!(MyEnum::Variant2(()).variant_name(), "Variant2");
    assert_eq!(MyEnum::Variant3{_field1: ()}.variant_name(), "Variant3");
}
