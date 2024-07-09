# Chapter 5: Using Structs to Structure Related Data

### Defining Structs:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Spread can also be done for structs:
```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit like Structs
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

For a struct to store references to data owned by something else, we will require the use of Lifetimes.

Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`). 


## Methods
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object and their first parameter is always `self`
```rust 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
It is legal to have multiple `impl` blocks for the same struct

In C/C++ `.` is used for calling a method directly on an object. `->` is used when calling the method on a pointer. However Rust does this automatically. Hence `p1.distance(&p2);` and `(&p1).distance(&p2);` are the same

### Associated Functions
Functions defined with `impl` are associated functions