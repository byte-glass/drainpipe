# drainpipe

## latest

 - bin/pipeline.rs
 - working version in src/main.rs

## next steps

 - tidy up pipeline ??

## usage

To run `./src/main.rs`,

```sh
cargo run --bin drainpipe
```

## read a file

What is the effect of the block around `let mut file ... `?

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
