# Chapter 2: Programming a Guessing Game

### Guessing Game
* Rust variables are immutable by default
```rust
let apples = 5;
```
Mut keyword makes them mutable
```rust
let mut apples = 5;
```

* Dependencies can be added to Cargo.toml
```rust
[dependencies]
rand = "0.8.5"
```

* Cmp package is used for the string comparison
```rust
use std::cmp::Ordering;
```

* Rust allows you to shadow prev values of variables ie Multiple varaibles 
can be created with the same name (Useful for type conversion)

* Use **match** expressions to handle errors
    - Match statements have arms
    - Each arm is a pattern to match against