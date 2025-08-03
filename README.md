## Get started
```
cargo new my_project
```
### Dependencies
```
cargo add macroquad
cargo add rand
```
Replace the code main.rs then
```
cargo run
```
Note: You can paste all the codes into src/bin/ and run it individually using 
```cargo run --bin code_name```

### Building a .exe file
Macroquad allows to build .exe files with no additional dependencies
```
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu
```

Macroquad: https://github.com/not-fl3/macroquad

