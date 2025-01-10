### Here are some Rust crates I've developed for different tasks in my projects. Each one is made to simplify specific functionalities.


- [**Enum Variant Name**](./enum_variant_name/README.md): Provides a convenient derive macro to retrieve the variant name of an enum in Rust.

- [**Logger**](./logger/README.md): A multilogger based on the [log](https://docs.rs/log) crate

- [**Mem**](./mem/README.md): Nothing really interesting here for now.

- [**Networking**](./networking/README.md): Simplifies TCP connections with socket-style wrapper around `std::net::TcpStream` and a proxy mechanism that offers basic stats calculation for round-trip time and bytes exchanged.

- [**Random**](./random/README.md): Simple randomisation api for games, includes a weighted bag system for drop tables.

- [**Threading**](./threading/README.md): Channels, Threadpools and sync futures.

- [**Time**](./time/README.md): Delta time based delay, stopwatch, function exec timing and time formatting.


Please check each crate's readme for more detailed information on their usage.


### Note on versioning
These crates do not use crate traditional version numbers.   
To select a specific version, please use the git commit hash in your `Cargo.toml` file like so:

```toml
[dependencies]
time = {git = "https://github.com/Bowarc/Crates.git", package = "time", rev = "b08aab9"}
```

