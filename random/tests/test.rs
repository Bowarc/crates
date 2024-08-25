#[test]
fn get() {
    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    let _ = random::get(0, 10);

    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    let _ = random::get_inc(0, 10);
}

#[test]
fn pick(){
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // random item in v
    let item = random::pick(&v);

    assert!(v.contains(item))
}

#[test]
fn _str(){
    assert_eq!(random::str(50).len(), 50)
}

