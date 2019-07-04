
## Usage
In Cargo.toml:
```
[dependencies]
simple-timer: { git = 'https://github.com/hwchen/simple-timer-rs' }

```

In code:
```rust
use simple_timer::timeit;

fn hello_world() {
    println!("hello world");
}

fn main() -> {
    timeit!("time_1", hello_world());
    timeit!("time_two",
        {
            println!("great weather");
            println!("i agree");
        }
    );
}
```
