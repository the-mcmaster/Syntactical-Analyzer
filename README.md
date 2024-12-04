**Author:** Eric Ovenden

# Question 1: Lexical Analysis

This is the codebase for question 1. It features a state machine that reads the bytes of a file onbe-by-one, and spits out token-lexeme pairs as it goes along.

### Task 3.1
1. `Develop a lexical analyzer for the given C-like code using a finite state machine model as discussed in the class and the textbook.`
   - See `StateMachine` in `src/lexer.rs`, especially the method `tick`
2. `You should print all lexemes and corresponding tokens.`
   - See `main` in `src/main.rs`
3. `Note that your lexical analyzer should consider all white space characters including, space, tab, and line break.`
   - See `is_whitespace` in `src/lexer.rs`
4. `Also, note that the lexical analyzer should read one character at a time.`
   - See `open_file` in `src/io.rs`

#### Assumptions
1. All literals are categorized as an integer first, then promoted to a float.
2. There can be any whitespace after any valid token.
3. Symbol tokens are always 1 character long.
4. `int`, `float`, and `return` are reserved and cannot be an identifier.
5. Whitespace and symbols will always terminate a token.
6. Whitespace can be included between any two tokens.

# Dependencies
This relies only on the standard library.

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
[hostname@user Q1]$ cargo run -- ../targets/test.txt
   Compiling Q1 v0.1.0 (/path/to/ProgrammingAssignment2/PA2/Q1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/Q1 ../targets/test.txt`
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