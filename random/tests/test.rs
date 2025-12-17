#[test]
fn get() {
    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    let _ = random::get(0, 10);

    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    let _ = random::get_inc(0, 10);
}

#[test]
fn pick() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // random item in v
    let item = random::pick(&v);

    assert!(v.contains(item))
}

#[test]
fn _str() {
    assert_eq!(random::str(50).len(), 50)
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn deterministic() {
    const SEED: u64 = 8090289391169979980;

    fn assert_int_sequence<
        T: Clone + std::fmt::Debug + PartialOrd + rand::distr::uniform::SampleUniform,
    >(
        seq: &[T],
        min: T,
        max: T,
    ) {
        for s in seq.iter() {
            assert_eq!(s, &random::get(min.clone(), max.clone()))
        }
    }

    fn assert_bool_sequence(seq: &[bool]) {
        for b in seq.iter() {
            assert_eq!(b, &random::conflip())
        }
    }

    assert_ne!(SEED, random::seed());

    random::set_seed(SEED);

    assert_eq!(SEED, random::seed());

    #[rustfmt::skip]
    assert_int_sequence(
        &[
            2048557442, // 1473658190,
            1762536587, // 1355345873, 
            944008383, // 131327370, 
            1160680275, // 1937881571, 
            1426541036, // 31378253,
            137237743, // 517943301,
            889800351, // 7043390,
            1847050577, // 233275587,
            2108763530, // 1545832033,
        ],
        0,
        i32::MAX,
    );

    assert_eq!(5, random::get::<i32>(0, 10));
    assert_eq!(3274, random::get::<i32>(42, 10000));
    assert_eq!(10, random::get_inc::<i32>(0, 10));
    assert_eq!(3830, random::get_inc::<i32>(42, 10000));
    assert_eq!(326153804, random::get_inc::<i32>(0, i32::MAX));

    assert_eq!(5, random::get::<u128>(0, 10));
    assert_eq!(2059, random::get::<u128>(42, 10000));
    assert_eq!(7, random::get_inc::<u128>(0, 10));
    assert_eq!(6974, random::get_inc::<u128>(42, 10000));
    assert_eq!(8, random::get_inc::<u128>(0, 10));
    assert_eq!(
        277383218987136418957275026246268235247,
        random::get_inc::<u128>(42, u128::MAX)
    );

    assert_eq!(
        "DoIMW8hX9OWEYxQEBJj9cXBHK9Gd0sl0pZQJFIzcwJrwmU1vagFASixQaJYW6toAjtnIrMeZgmLV2fHmiEACDCnN3qr6eLLM9u1v",
        random::str(100)
    );

    #[rustfmt::skip]
    assert_bool_sequence(&[
        true, true,
        false, false,
        true, true,
        true, true,
        true, false,
        true, true,
        false, true,
        false, true,
        true, false,
        true, true,
        false, true,
        false, false,
        true, false,
        true, false,
    ]);
}
