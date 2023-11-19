## Simpe random number and string generation

#### Use example:

cargo.toml
```toml
[dependencies]
logger = {git = "https://github.com/Bowarc/Crates.git", package = "random"}
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
``` 