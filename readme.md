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

## style 'pipeline'

this uses iterators over or vectors of `String`. the more monolithic (i.e. one lifetime?) version in main.rs can use `&str`, the references are good within the single scope -- try to understand this better!

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
