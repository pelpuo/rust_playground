# Chapter 4: Understanding Ownership

## Ownership
- Rules governing how a Rust program manages memory

### Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### String type
- `let mut s = String::from("hello");`
- `s.push_str(", world!");` Can be used to add characters to s
- The memory is freed using `drop()` once `s` goes out of scope

### Variable and Data interaction
```rust
let x = 5;
let y = x;

```
For this instance, a copy of the value in x (5) is created and assigned to y


```rust
let s1 = String::from("hello");
let s2 = s1;

```
For this instance s1 which is {ptr, len, capacity} is duplicated and assigned to s2 ie they both point to the same location on the heap.

#### However:
- If Rust attempts to free both `s1` and `s2` when they go out of scope, it would be a double free
- Rust invalidates `s1` when `s2` is created to prevent this
- Becomes a `move` instead of a `shallow copy`
- The `copy` anootation can be used 

<img src="/home/ekayang/Documents/Github/rust_playground/4_understanding_ownership/trpl04-04.svg" alt="String assignment" width=50%/>

- To implement a deep copy, use clone:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

### Ownership and functions
- Same applies when passing values to functions.
- Values typically on the heap are moved into functions
    - The original variable is invalidated
- Stack values are copied and can still be used after the function


## References and Borrowing
- We can pass a reference to functions to allow us to use a variable after it has been passed as an argument to a function

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```

- `s` becomes a reference to `s1` but does not own it. `s1` is not dropped.
- This action is called **borrowing**
- References are immutable by default. 
    - You can have an unlimited number of immutable references
- They can be made mutable if required:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
- However: if you have a mutable reference to a value, you can have no other references to that value.(This prevents data races).
- You also cannot combine mutable and immutable references. 

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");

```
- This code will work however, since `r3` comes after the last use of `r1` and `r2`


## Slice Type
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

```
### String object vs String literal
- `let s = String::from("Hello world")` is a string object which can be mutated. It is also allocated on the heap
- `let s = "Hello world"` is a hard coded string which is immutable and hard coded into the program (Not allocated on the heap)