# Hello World : A RustLang library

This Rust project provides a simple greeting function that greets a person by name or defaults to "Hello World!" if no name is provided.

## Usage

To use this library in your Rust project, add it as a dependency in your `Cargo.toml`:
```toml
[dependencies]
ludndev-hello-world = "0.1.0"
```

Then, you can use it in your code:

```rust
use ludndev-hello-world::greet;

fn main() {
    let name = "John";
    let greeting = greet(name);
    println!("{}", greeting);
}
```

## API

### `greet(name: &str) -> String`

This function takes a `name` as input and returns a greeting message. If the `name` is empty, it defaults to "Hello World!".

## Examples

```rust
use ludndev-hello-world::greet;

fn main() {
    let name = "Alice";
    let greeting = greet(name);
    assert_eq!(greeting, "Hello Alice !");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

This project was authored by [Judicaël AHYI](https://github.com/ludndev).
