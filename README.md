# [Advent of Code 2021](https://adventofcode.com/2021)

Use `cargo-aoc` to run:
```
cargo install cargo-aoc
cargo aoc -d [day] -p [part]
```

# Tricks

## [Recap crate](https://github.com/softprops/recap)

Parse a string into a struct using a Regex
```rust
use recap::Recap; 
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<min>\d+)-(?P<max>\d+) (?P<chr>[a-z]): (?P<word>[a-z]+) / (?P<csv>.*?)$")]
pub struct MyStruct {
    min: usize,
    max: usize,
    chr: char,
    word: String,
    csv: Vec<String>,  // supports parsing comma separated values as a Vec<T>
}

fn main() {
    let my_struct = "01-99 x: hello / I,am,a,Vec".parse::<MyStruct>();
    println!("{:?}", my_struct)
    // Output: Ok(MyStruct { min: 1, max: 99, chr: 'x', word: "hello", csv: ["I", "am", "a", "Vec"] })
}
```

Also composable with enums:
```rust
use recap::Recap; 
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Deserialize, Recap)]
#[recap(regex = r#"(?P<dir>\w+) (?P<dist>\d+)"#)]
pub struct MyStruct {
    dir: Direction,
    dist: u32,
}

fn main() {
    let my_struct = "forward 123".parse::<MyStruct>();
    println!("{:?}", my_struct)
    // Output: Ok(MyStruct { dir: Forward, dist: 123 })
}
```
