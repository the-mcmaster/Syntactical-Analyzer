**Author:** Eric Ovenden

# Question 2: Syntax Analysis

This is the codebase for question 2. It features an encapsulation parse tree design pattern, with
a left recursive-descent parsing strategy.

### Task 4.1

1. `Construct a parse tree for the provided function using a recursive descent parser.`
   - See `Parse` in `src/lib.rs`
2. `This tree should visually represent the syntactic structure of the code as dictated by the grammar of the language.`
   - See `modulars.rs`, `non_terminals.rs`, and `terminals.rs` in `src/`
3. `Please provide separately the BNF/EBNF grammar that you use in your code.`
   - See `BNF.md` in this crate's root
   - BNF snippets also included for each parsing type in the library documentation.

### Task 4.2
1. `Develop a C-like grammar in BNF/EBNF for the provided code snippet. (20 pts)`
   - See `BNF.md` in this crate's root
2.
- `Develop a recursive descent parser and a parse tree starting from the function definition down to the terminals lexemes.`
   - See `main` in `src/main.rs`
   - See `Parse` in `src/lib.rs`

- `Print the parse tree the way it is printed in the book for the recursive descent parser example provided in the book.`
   - See `main` in `src/main.rs`
   - See `ParseDisplay` in `src/lib.rs`

- `You may assume that the right-hand side expression of an assignment statement will have...and 10 + 30 should also be legal.`
   - See `modulars.rs`, `non_terminals.rs`, and `terminals.rs` in `src/`

- `State all your assumptions explicitly.`
   - There can be zero or more function parameters for the function definition.
   - Function parameters are seperated by commas, and are strinctly *delimited* (not terminated) by it.
   - There can be zero or more statements within the compound statements.
   - Statements in a compound statement must always be terminated by a semicolon, no matter the statement type.
   - All assignment statements start with an identifier. No type information can be given.
   - Arithmetic expressions can be either a
      - singular identifier or literal
      - identifier(s) and literals(s) delimited by +, -, *, and/or /
   - Arithmetic expressions can have 0, 1, 2, or 3 operators. However, not all operator combinations are possible. Below are all possible valid parses (_ is a placeholder for literal/identifier).
      - _ (\*/) _ (+-) _ (\*/) _
      - _ (\*/) _ (+-) _
      - _ (+-) _ (\*/) _
      - _ (+-\*/) _
   - Arithmetic expressions has (\*/) lower than (+-) in the parse tree to enforce operator precendence.
   - Typecast expressions expect only an identifier for the casted value.

### Task 4.3
For the implementation for how the output is generated to `stdout`, see `ParseDisplay` in `src/lib.rs` and the corresponding implementations.

# Dependencies
1. `q1_lib` from the previous question (`Q1`)

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
```text
[hostname@user Q2]$ cargo run -- ../targets/test.txt
   Compiling Q1 v0.1.0 (/path/to/ProgrammingAssignment2/main/Q1)
   Compiling Q2 v0.1.0 (/path/to/ProgrammingAssignment2/main/Q2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/Q2 ../targets/test.txt`
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