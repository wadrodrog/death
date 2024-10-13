# death

A program that predicts your death date (Learning project)

![Demonstration of CLI death](death.png)

## Usage

### As program


```
death [OPTIONS]
```

### Options

```
  -n, --name <NAME>           Your name
  -b, --birthday <BIRTHDAY>   Your birthday
  -d, --death-reasons <FILE>  Custom death reasons file
  -h, --help                  Print help
  -V, --version               Print version
```

### As library

Add dependency to `Cargo.toml`:

```toml
[dependencies]
death = { git = "https://github.com/wadrodrog/death.git" }  # latest
death = { git = "https://github.com/wadrodrog/death.git", tag = "0.2.0" }  # from tag
```

`src/main.rs`:

```rust
use death::{date::Date, user::User};

fn main() {
    let user = User::new(1234567890, 45, vec![String::from("lego")]);
    println!("{}", Date::today());
    println!("{} - {}", user.get_death_date(false), user.get_death_reason());
}

```

Generate docs:
```shell
$ cargo doc --open
```

## Build

You need [Cargo](https://doc.rust-lang.org/cargo/index.html) to build this
program.

```shell
$ git clone https://github.com/wadrodrog/death.git
$ cd death
$ cargo build
$ ./target/debug/death
```
