# Chapter 3: Common Programming Concepts

## Variables and Mutability

### Mutability vs Immutability
- `let x = 5;` is an immutable variable 
- `let mut x = 5;` is a mutable variable 

### Constants
- Always immutable
- Type needs to be declared
- Example: `const THREE_HOURS_IN_SECONDS:u32 = 60 *60 * 3;`
- Rust can eval this operation at compile time

### Shadowing
- Can be used to create variable scopes
- Allows for redeclaration of immutable vars (Require `let` at every instance)
- Allows us to redeclare a variable with a different type


## Data Types

### Integer Types
- i<size>, u<size> [8, 16, 32, 64, 128] 
- Literals:
    - Decimal 98_222 (**_** is a visual separator)
    - Hex 0xff
    - Octal 0o77
    - Binary 0b1111_0000
    - Byte b'A' (u8 only)
- Rust checks for integer overflows in debug mode
- When you compile with `--release` the checks are not included

#### Handling Overflows
- `wrapping_*` methods to eg `wrapping_add`
- `checked_*` return none if overflow
- `overflowing_*` value + boolean indicating overflow
- `saturating_*` for capping values

### Floating-Point Types
- f32, f64

### Boolean types
- `let f: bool = false;`
- Explicit type annotation is not necessary `let t = true;` is also valid

### Char type
- `let z = 'z';` or `let z: char = 'z';`
- Specified with single quotes

## Compound Types

### Tuple
- `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- `let (x, y, z) = tup;` can access individual elements
- `let five_hundred = tup.0;` can be used to access individual elements

### Array Type
- `let a = [1,2,3,4,5];`
- `let a: [i32;5] = [1,2,3,4,5];` to specify type
- `let a = [3;5];` for initialization (5 elements initially set to 3)
- `let first = a[0];` to access elements

## Functions
- `fn a(x:i32){}` For specifying parameters

#### Using a block for assignment:
```rust
let y = {
    let x = 3;
    x + 1
};
```
#### Specifying return values:
Return statements do not require a semicolon at the end of the line. The semicolon
changes the line from an expression to a statement (statements have no return val)
```rust
fn five() -> i32{
    5
}
```


## Control Flow

### Conditionals
```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

```
- Condition must be a boolean otherwise we will get an error.
- Use `if number != 0{}` and not `if number {}`
- Conditionals can also be used in let: `let number = if condition { 5 } else { 6 };` However, type has to stay consistent


### Looping 

#### Basic Loop
```rust
loop {
    println!("again!");
}
```

#### Returning Values
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
- **break** exits loop but **return** exits the current function
- Loops can be labeled: `'counting_up: loop{}`

#### While Loop
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

#### For Loop
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```