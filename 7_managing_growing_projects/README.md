# Managing Growing Projects with Packages, Crates and Modules

## Packages and Crates

### Crate
- Binary crates are compiled to executable and contain `main`
- Library crates don't have `main` but define functionality to be shared with multiple projects

#### Creating Crates
```bash
$ cargo new my-project
     Created binary (application) `my-project` package
```

### Keywords
- `use` brings a path into scope
- `pub` makes items public
- `as` changes name of module


## Modules
Allows us to organize code within a crate for readability and reuse

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}
```

When declaring a path, using `super` is similar to navigating to parent (`..`).

If a Struct within a module is made public, each individual element needs to be made public to be accessed externally.
If an Enum is made public, all elements are automatically made public.

The `use` keyword can be used instead of always delcaring paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

For the `use` keyword, the convention is to declare the path up to the module. This would make it clear that the function is from a different module.

### Using `as`
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

When you want an imported module to be available to files which import your module use `pub use` rather than just `use`

```rust 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

External packges are added to **Cargo.toml**

Paths can be nested
```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

Glob (`*`) brings all into scope
```rust
use std::collections::*;
```