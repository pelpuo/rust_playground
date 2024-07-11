# Chapter 8: Commmon Collections

## Vectors
```rust 
let v: Vec<i32> = Vec::new(); //or
let x: Vec<i32> = vec![1, 2, 3];
```

### Reading Elements
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

Using an enum can allow you to store different types in a vector

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## Strings
- Defined in rust: `let mut s = String::new();`
- Use `to_string();` to convert characters into a Rust string
- Use `push_str()` to append to a string
- `push()` appends a single character
- Strings can also be concatenated with `+`
- Rust does not support string indexing to account for var sized characters
    - You can instead access chars `for c in "Зд".chars(){}` or bytes `for c in "Зд".bytes(){}`

## Hash Maps
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Accessing values
let score = scores.get(&team_name).copied().unwrap_or(0);

// Iterating
for (key, value) in &scores {
    println!("{key}: {value}");
}

// Add if not present
scores.entry(String::from("Yellow")).or_insert(50);
```

By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
