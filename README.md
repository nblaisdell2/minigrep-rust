# Simple GREP CLI tool

This program is a simple CLI tool that simulates the common "grep" program, by searching for a particular "search" string in a particular file, and returning each line in that file that contain the "search" string.

This program was built by following the tutorial in the "[Rust Programming Language Book (2021 Edition)](https://nostarch.com/rust-programming-language-2nd-edition)" book, from Chapter 12: An I/O Project: Building a Command Line Program

- [Online Tutorial](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

## How to Use

#### 1. Build

To gain access to the program from the source code, you can run the following command to build the Rust source code into a binary/executable that can be run from the command line:

```
cargo build
```

#### 2. Run

From there, you can use the binary/executable directly, or you can use the `cargo` command to run it.

The program expects two parameters:

1. **Search String**
   - This is a string which the user is attempting to search for in the given file
   - The program will search the file for any lines that contain this search string, and return them to the standard output (`stdout`)
2. **File Path**
   - This is the path to the text file to search through

```
# Using Cargo
cargo run -- searchstring /path/to/file.txt

# Using directly
minigrep-rust searchstring /path/to/file.txt
```
