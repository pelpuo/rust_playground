# Chapter 9: Error Handling

## Using panic!
`panic!` macro stops execution with error message.

```rust
fn main() {
    panic!("crash and burn");
}
```

`RUST_BACKTRACE=1 cargo run` allows you to get a panic backtrace.

## Using Result
Definition of Result
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Use 
```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

Concise way of handling. `unwrap()` returns value inside `Ok()`.
```rust
let greeting_file = File::open("hello.txt").unwrap();

// or
let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
```

