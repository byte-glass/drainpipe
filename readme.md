# drainpipe

## latest

 - continuation passing is doomed
 - bin/infinite_mirror.rs
 - bin/pipeline.rs
 - working version in src/main.rs

## next steps

 - try `things`
 - bin/the_one.rs - difficult
 - tidy up pipeline ??

## usage

To run `./src/main.rs`,

```sh
cargo run --bin drainpipe
```

## todo

 - counts as `reduce`: apply `reduce` to iterator with closure over the hashmap?
 - try pipeline with iterators over &str, this would require sorting out lifetimes!

## continuation passing i.e. style 'kick-forward'

the signature of the continuation must (?) reach all the way to the end of the computation!? this is not practical!!

## style 'golf'

```rust
let s = fs::read_to_string("./stop_words.txt").expect("can't find stop_words.txt?");
let ascii = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",  "y", "z"];
let stop_words = s.split(",").chain(ascii).collect::<Vec<&str>>();
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
