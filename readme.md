# drainpipe

## latest

 - working version in src/main.rs

## next steps

 - pipeline

## usage

To run `./src/main.rs`,

```sh
cargo run --bin drainpipe
```

## read a file

A more long-winded alternative to `fs::read_to_string(filename).expect("file not found?");`

```rust
use std::{fs::File, io::Read};

let filename = "pride-and-prejudice.txt";
let mut data = String::new();
{
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut data).unwrap();
}
```

Use of `.expect` ??


### end
