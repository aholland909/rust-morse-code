# Rust Morse Code Encoder
Convertor for strings into morse code

## Building
```
cargo build
```

## Running
```
cargo run "input string" 
```



## Usage
Normal
```
$ cargo run "hello world" 

Input: hello world
Output: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```
## Flags 
- Output sound flag -s 

With sound output 
```
$ cargo run "sounds output" -s

Input: sounds output
Output: ... --- ..- -. -.. ... / --- ..- - .--. ..- -
```
