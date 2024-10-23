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
        T: Clone + std::fmt::Debug + PartialOrd + rand::distributions::uniform::SampleUniform,
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
            1473658190,
            1355345873, 
            131327370, 
            1937881571, 
            31378253,
            517943301,
            7043390,
            233275587,
            1545832033,
        ],
        0,
        i32::MAX,
    );

    assert_eq!(4, random::get::<i32>(0, 10));
    assert_eq!(1862, random::get::<i32>(42, 10000));
    assert_eq!(0, random::get_inc::<i32>(0, 10));
    assert_eq!(188, random::get_inc::<i32>(42, 10000));
    assert_eq!(2046971347, random::get_inc::<i32>(0, i32::MAX));

    assert_eq!(7, random::get::<u128>(0, 10));
    assert_eq!(8716, random::get::<u128>(42, 10000));
    assert_eq!(5, random::get_inc::<u128>(0, 10));
    assert_eq!(3, random::get_inc::<u128>(0, 10));
    assert_eq!(1, random::get::<u128>(0, 10));
    assert_eq!(2323, random::get_inc::<u128>(42, 10000));
    assert_eq!(
        240865189765160804535080564014790761235,
        random::get_inc::<u128>(42, u128::MAX)
    );

    assert_eq!(
        "ek1VFAPH4iHlfYG9SyPL2H3eleUK0JGyGz89PZLkFGT39shBCkWyMK83QCM6vAWgGkp6cGTzznbm8B3WIajfWCugTm502wUDSwzA", 
        random::str(100)
    );

    #[rustfmt::skip]
    assert_bool_sequence(&[
        true, false,
        false, true,
        true, true, 
        false, true,
        false, true,
        false, true,
        true, false,
        true, true, 
        true, true, 
        false, true
    ]);
}
