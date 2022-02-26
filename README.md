Parge is a simple and easy to use argument parser

# Features
* Long and short varients
* Switch options
* Value options

## Switch options
Options that are a simple on/off switch

`example --option`

## Value options
Options that contain some data

`example --hello world`

# Usage

```rust
use parge::parser::Parser;

fn main() {
    let mut switch = false;

    let args = Parser::new()
        .add_switch_opt(
            Some("option"),
            Some('o'),
            &mut switch)
        .parse();

    println!("{}", switch);
}
```
