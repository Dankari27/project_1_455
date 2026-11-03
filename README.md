Richard Savoca

COSC 455 - Project 1

Repo: https://github.com/Dankari27/project_1_455

Operating System Target:
Windows 11 64 bit


## Setup

### 1. Install Rust

This project uses the Rust 2024 edition, so it needs **Rust 1.85 or newer**.

# 1. Instructions 
```
Download and run `rustup-init.exe` from https://rustup.rs
```
### 2. Get the code

```
git clone https://github.com/Dankari27/project_1_455
cd project_1_455
```

### 3. Build

```
cargo build --release
```

The program is built to `target\release\lolcompiler.exe`.


### 4. Run

Give the compiler a `.lol` file (files are in lolcompiler\test):

```
cargo run --release -- test\Test2.lol
```

or run the exe directly:

```
target\release\lolcompiler.exe test\Test2.lol
```

### 5. Run the tests

```
cargo test
```

