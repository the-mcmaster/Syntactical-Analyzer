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
 Documenting Q2 v0.1.0 (path/to/crate)
    Checking Q2 v0.1.0 (path/to/crate)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
   Generated path/to/crate/target/doc/q2_lib/index.html and 1 other file
```

Using your browser of choice, you can then open the html file at `[/path/to/crate/]target/doc/Q2/index.html`.

### Raw Documentation Comments (not recommended)
All documentation is written in the source code with comment lines starting in `/// This is a documentation line`. Documentation comments can be read in markdown style (*that's how `cargo doc` knows how to format the HTML*).

# Crate Organization

### High Level Overview

This crate is split into two parts: a **binary** part and a **library** part, which is reflected in `Cargo.toml`.

```
Q2
|\_ src
|   |\_ lib.rs            <-|
|   |                       |
|   |\_ modulars.rs       <-|--- Library (q2_lib)
|   |                       |
|   |\_ non_terminals.rs  <-|
|   |                       |
|   |\_ terminals.rs      <-|
|   |
|   \_ main.rs <-------- Binary  (Q2)
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

The library is split into three modules,
- `terminal.rs`: All terminal parse types
- `non_terminal.rs`: All composite parse type (all items built off of the terminal primatives).
- `modular.rs`: Handles special list-like BNF grammars.

##### Note to the grader...
To preform recursive-decent parsing (an LL parser implementation specifically),
this library relies on shared behavior from the `Parse` trait. Similarly,
`ParseDisplay` is equally as important for reporting the parse tree to stdout.

### Binary

The binary part code is simply `main.rs`, and is very small. After getting the
lexemes from the input file, it will try to parse the token stream.

If success, it prints out the parse tree.

If failure, it prints out an error message and exits.

# Expected Output
When ran on with the provided code, it should return
```
[hostname@user Q2]$ cargo run -- -i ../targets/test.txt
   Compiling Q1 v0.1.0 (/path/to/ProgrammingAssignment2/main/Q1)
   Compiling Q2 v0.1.0 (/path/to/ProgrammingAssignment2/main/Q2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/Q2 -i ../targets/test.txt`
Function Definition: int foo (float x, int y) {....}
    Funtion Return Type: int
    Function Identifier: foo
    Left Paren: (
    Function Parameters: float x, int y
        Function Parameter: float x
            Parameter Type: float
            Parameter Identifier: x
        Function Parameter: int y
            Parameter Type: int
            Parameter Identifier: y
    Right Paren: )
    Left Curly: {
    Compound Statements: y = x + 10; x = y / 2.0; y = (int)x; return x;
        Statement:
            Assignment Statement: y = x + 10
                Identifier: y
                Equals: =
                Expression:
                    Arithmetic Expression: x + 10
                        Term: x
                            Factor: x
                                Variable: x
                        Operator: +
                        Term: 10
                            Factor: 10
                                Literal: 10
        Statement:
            Assignment Statement: x = y / 2.0
                Identifier: x
                Equals: =
                Expression:
                    Arithmetic Expression: y / 2.0
                        Term: y / 2.0
                            Factor: y
                                Variable: y
                            Operator: /
                            Factor: 2.0
                                Literal: 2.0
        Statement:
            Assignment Statement: y = (int)x
                Identifier: y
                Equals: =
                Expression:
                    Typecast Expression: (int)x
                        Left Paren: (
                        Cast Type: int
                        Right Paren: )
                        Cast Indentifier: x
        Statement:
            Return Statement: return x
                Return: return
                Expression:
                    Arithmetic Expression: x
                        Term: x
                            Factor: x
                                Variable: x
    Right Curly: }

```