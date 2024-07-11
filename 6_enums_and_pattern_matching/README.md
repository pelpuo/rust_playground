# Chapter 6: Enums and Pattern Matching

## Defining Enums
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Data can be put into each enum variant
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```


## The Option Enum
Encodes scenario where a value can be something or nothing.

```rust
//Defined by the standard library as
enum Option<T> {
    None,
    Some(T),
}

// How to use Option
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

`<T>` means that the Some variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T> `type a different type.


## Match
Works like Switch in C/C++
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 0
    }
}
```
Matches are exhaustive so all possible cases for a value must be covered.
`_` or `other` can be used like a default case. However `other` can bind the value to itself.

## If let
Combines if and let. Can be used when you just want to match one value and ignore the rest.

```rust
let coin = Coin::Penny;
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

