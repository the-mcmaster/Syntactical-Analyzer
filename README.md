**Author:** Eric Ovenden

# Question 1: Lexical Analysis

This is the codebase for question 1. It features a state machine that reads the bytes of a file onbe-by-one, and spits out token-lexeme pairs as it goes along.

The library (`q1_lib`) is reused as a dependency in Q2.

# Executing

To execute the binary, simply run
`cargo run -- -i [path/to/c_like/file]`

This will
1. compile the source code, and execute it
    - (`cargo run`)
2. and pass in the arguments `-i [path/]` to the binary
    - (`-- -i [path/]`)

# Documentation

Two options are available to read the documentation.

### Well-Formatted HTML (recommended)
To view an well-formated HTML formatted website for quick navigation of the source code, in the crate's root, run 

`cargo doc --document-private-items --open`

This will
1. compile the documentation into a well-formatted HTML page,
    - (`cargo doc`)

2. including all `private`/`pub(crate)` items
    - (`--document-private-items`),

3. and then try to open it in a web browser.
    - (`--open`)

#### If for some reason it cannot open to a web browser automatically...

... simply truncate the `--open` flag:

`cargo doc --document-private-items`

Using this command instead, you will see output to stderr that look like this:
```
 Documenting Q1 v0.1.0 (path/to/crate)
    Checking Q1 v0.1.0 (path/to/crate)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
   Generated path/to/crate/target/doc/q1_lib/index.html and 1 other file
```

Using your browser of choice, you can then open the html file at `[/path/to/crate/]target/doc/Q1/index.html`.

### Raw Documentation Comments (not recommended)
All documentation is written in the source code with comment lines starting in `/// This is a documentation line`. Documentation comments can be read in markdown style (*that's how `cargo doc` knows how to format the HTML*).

# Crate Organization

### High Level Overview

First and foremost: **~80% of the codebase is in `lexer.rs`.**

This crate is split into two parts: a **binary** part and a **library** part, which is reflected in `Cargo.toml`.

```
Q1
|\_ src
|   |\_ lib.rs    <-|
|   |               |
|   |\_ io.rs     <-|--- Library (q1_lib)
|   |               |
|   |\_ lexer.rs  <-|
|   |
|   \_ main.rs <-------- Binary  (Q1)
|
|\_ Cargo.toml
|
|\_ Cargo.lock
|
|\_ README.md
```

`src/`: All source code, library and binary.

`Cargo.toml`: A file describing the crate structure for `cargo`, and other metadata.

`Cargo.lock`: A file managed by `cargo`. Manual editing is not recommended.

`README.md`: You are *here*.

### Library

The root of the library is at `lib.rs`.

The library is split into two modules,
- `io.rs`: All important IO related functionality
- `lexer.rs`: All lexical analysis functionality, structs, and enums.

Most of the library's code is under `lexer.rs`.

##### Note to the grader...
Most of the code base is the implementation of `StateMachine`'s method `.tick(...)` within the `lexer.rs` module. This is the state machine's driver as it "ticks" through the bytes of the input file.

### Binary

The binary part code is simply `main.rs`, and is very small as it simply "gets" the lexemes from the input file and nicely prints the results. You will find the "entrypoint" into the library is with the function `get_lexemes()`.

# Expected Output
When ran on with the provided code, it should return
```
[eric@elinux Q1]$ cargo run -- -i ../targets/test.txt
   Compiling Q1 v0.1.0 (/home/eric/Desktop/ProgrammingAssignment2/main/Q1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/Q1 -i ../targets/test.txt`
TOKEN                   |LEXEME
________________________|________________________
Type(Int)               |int
Identifier              |foo
Symbol(LeftParen)       |(
Type(Float)             |float
Identifier              |x
Symbol(Comma)           |,
Type(Int)               |int
Identifier              |y
Symbol(RightParen)      |)
Symbol(LeftCurly)       |{
Identifier              |y
Symbol(Equal)           |=
Identifier              |x
Symbol(Plus)            |+
LiteralInt              |10
Symbol(Semicolon)       |;
Identifier              |x
Symbol(Equal)           |=
Identifier              |y
Symbol(Divide)          |/
LiteralInt              |2.0
Symbol(Semicolon)       |;
Identifier              |y
Symbol(Equal)           |=
Symbol(LeftParen)       |(
Type(Int)               |int
Symbol(RightParen)      |)
Identifier              |x
Symbol(Semicolon)       |;
Return                  |return
Identifier              |x
Symbol(Semicolon)       |;
Symbol(RightCurly)      |}

```