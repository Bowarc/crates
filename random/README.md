## Simple random number and string generation

### Documentation

The documentation for this crate can be found [here](https://bowarc.github.io/crates/random)

#### Use example:

Cargo.toml
```toml
[dependencies]
random = {git = "https://github.com/Bowarc/Crates.git", package = "random"}
``` 
main.rs
```rust
// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
let _ = random::get(0, 10);

// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
let _ = random::get_inc(0, 10);

let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// random item in v
let _v_item = random::pick(&v);

// 50% chance true, 50% false
let _coin = random::conflip();

// Random string for given len
let r = random::str(50);
println!("{r}") // kpk6PmI3pak70NMSWZLMO0DGAT5i0WzNtZWNbkNEPZbDbxlWDC

//
// Weighted bag //
//

// Create the bag

// From Vec<(T, Weight)>
// You can use any unsigned integer for Weight
let bag = random::WeightedBag::<&str, u8>::from(vec![
    ("Hi", 2), // ~28%
    ("Hellow", 1), // ~14%
    ("Bonjour", 4), // ~57%
]);

// Or from an empty one and you add the entries yourself
let mut bag = random::WeightedBag::<&str, u8>::default();

bag.add_entry("Hi", 2); // ~28%
bag.add_entry("Hellow", 1); // ~14%
bag.add_entry("Bonjour", 4); // ~57%

// And then, get a random one from the pool
assert!(&["Hi", "Hellow", "Bonjour"].contains(bag.get_random()));

// One thing to know, is that `bag.get_random()` will panic if there are no entries,
// if you're not sure, use bag.try_get_random() which returns a Option<&T> instead of &T w/ conditional panic
``` 
