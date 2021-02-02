# `startup`: Lightweight way to run code before main.

This crate is the moral equivalent to the `ctor` crate, although the API is completely different. The main reason for it's existence is:

- Much faster to compile — no proc macros / syn / quote.

- More obviously safe (avoids things I find dodgy, like `dtor`, ctors that initialize data, uses extern "C" function in function array called from C ...)

- Try to handle untested unix platforms by assuming they support *at least* the `.ctors` section. This is in line with what clang does to compile c++ static constructors.

## Usage

```rust
startup::on_startup! {
    // Note: not all of the rust stdlib may be supported before main.
    println!("I'm running before main");
}
fn main() {
    println!("I'm inside main");
}
```

Outputs:

```text
I'm running before main.
I'm inside main.
```
