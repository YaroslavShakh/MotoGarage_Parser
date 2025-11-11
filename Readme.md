# motogarage_parser

**Project Name:** motogarage_parser
**Description:** A parser and interpreter for MotoGarage DSL, a language for managing motorcycle collections.
**Crates.io link :** https://crates.io/crates/MotoGarage_parser
**Docs link :** https://docs.rs/MotoGarage_parser/0.1.3/motogarage_parser

---
## Technical Description of the Parsing Process

This project implements a parser and interpreter for a custom Domain-Specific Language (DSL) designed for managing a "garage" of motorcycles.

The data processing pipeline is as follows:

### 1. Input Data

The program accepts a text file (usually with a `.moto` extension) that contains a set of commands. There are three types of commands:
* `DEFINE` (for adding a motorcycle)
* `GET` (for querying the collection)
* `COUNT` (for counting items in the collection)

### 2. Parsing (Lexer + Parser)

* We use the `pest` library with a PEG (Parsing Expression Grammars) approach.
* The language grammar is defined in the `src/grammar.pest` file (see full grammar below). It describes the syntax: what "atoms" (numbers, strings, identifiers) exist and how they combine into "rules" (like `DEFINE`, `GET`, `COUNT`, `WHERE` clauses, etc.).
* `pest` automatically generates a parser (`MotogarageParser` in `src/lib.rs`) that reads the input string and builds a "tree of pairs" (Pairs), which accurately reflects the code's syntax while ignoring whitespace and comments (`//...`).

### 3. AST (Abstract Syntax Tree) Generation

* The "tree of pairs" from `pest` is generic. For practical use, we transform it into our own, strongly-typed **Abstract Syntax Tree (AST)**.
* Our AST structures (`Command`, `Motorcycle`, `Query`) are defined in `src/lib.rs`.
* The `parse_moto_file` function and its helpers (`parse_command`, `parse_definition`, etc.) recursively traverse the `pest` tree and build a `Vec<Command>`.

### 4. Interpretation (How the results are used)

* Once the input text is converted into an AST, we can "execute" it.
* The `Garage` struct (`src/lib.rs`) acts as the *interpreter*. It maintains the state (a `Vec<Motorcycle>`).
* The `Garage::execute` method iterates over the AST (`Vec<Command>`):
    * If the command is `Command::Definition`, it adds a new `Motorcycle` object to the internal list.
    * If the command is `Command::Get`, it calls the filtering logic (`run_query_get`), which checks the motorcycles against the query conditions and returns a list of names.
    * If the command is `Command::Count`, it calls the filtering logic (`filter_bikes`), counts the results, and returns a string with the total (e.g., "Bikes found: 2").

### 5. Output

* The binary file (`src/main.rs`) uses `clap` to parse CLI arguments.
* It reads the specified file, passes its content to `motogarage_parser::parse_moto_file`, receives the AST, passes the AST to `Garage::execute`, and prints the results returned by the interpreter (or any errors that occurred).

## CLI Usage

To compile and run the project:

```bash
# Ensure the code is formatted
cargo fmt

# Check the code for lints and style
cargo clippy

# Build the binary (in release mode)
cargo build --release

# Use `cargo run --` to run during development

# 1. Run the parser on a file 'my_garage1.moto': 
# There are  'my_garage2.moto' and 'my_garage3.moto'with different results to show parser work

cargo run -- parse my_garage1.moto

# 2. Display help 
cargo run -- --help
cargo run -- parse --help

# 3. Display credits
cargo run -- credits

```

---

## Grammar (src/grammar.pest)

Сomplete grammar used by the parser:

```pest
/// Ignores whitespace, tabs, and newlines
WHITESPACE = _{ " " | "\t" | "\n" | "\r" | line_comment }

/// Ignores single-line comments (from // to end of line)
line_comment = _{ "//" ~ (!"\n" ~ ANY)* }

// --- Top-Level Rules ---

/// A file is a Start of Input (SOI), followed by zero or more commands,
/// and ending with an End of Input (EOI).
file = { SOI ~ (command)* ~ EOI }

/// A command is either a definition (DEFINE), a GET query, or a COUNT query.
command = { definition | query_get | query_count }

// --- 1. Definition (DEFINE) Rules ---

/// Defines a new bike.
/// Example: DEFINE bike "Honda CBR" { year: 2021 }
definition = { "DEFINE" ~ "bike" ~ string_literal ~ "{" ~ properties ~ "}" }

/// A list of one or more properties, separated by commas.
properties = { property ~ ("," ~ property)* }

/// A single 'key: value' pair.
/// Example: year: 2021
property = { ident ~ ":" ~ value }

// --- 2. Query Rules ---

/// A 'GET BIKES' query, which can have an optional 'WHERE' clause.
query_get = { "GET" ~ "BIKES" ~ (where_clause)? }

/// A 'COUNT BIKES' query, which can have an optional 'WHERE' clause.
query_count = { "COUNT" ~ "BIKES" ~ (where_clause)? }

/// An optional 'WHERE' clause containing a single condition.
where_clause = { "WHERE" ~ condition }

/// A filter condition 'key <operator> value'.
/// Example: type = sport
condition = { ident ~ operator ~ value }

/// A comparison operator.
operator = { "=" | ">" | "<" }

// --- Basic Building Blocks (Atoms) ---

/// An identifier (a field name or type).
/// Starts with a letter or '_', followed by letters, numbers, or '_'.
ident = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

/// A value, which can be a number with unit (cc), a plain number,
/// a quoted string, or an identifier (like 'sport').
value = { number_with_unit | number | string_literal | ident }

/// A string of text in double quotes.
/// Example: "Honda CBR600RR"
string_literal = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

/// An integer.
/// Example: 2021
number = @{ ASCII_DIGIT+ }

/// A number with a 'cc' suffix (for engine displacement).
/// Example: 599cc
number_with_unit = @{ number ~ "cc" }

```
# Makefile Documentation

## Available Commands

### Main Quality Check

This is the most important command to run before committing new code.

* `make check`
    Runs the full quality-check suite. It executes `fmt`, `clippy`, and `test` all in one go.

### Running the Program

* `make run ARGS="..."`
    Runs the program in **debug** mode (for development).
    
    **Important:** You **must** provide the `ARGS` variable to pass arguments to the program.

    **Examples:**
    ```bash
    # Run the parser on a file
    make run ARGS="parse my_garage1.moto"
    
    # Show the credits
    make run ARGS="credits"
    ```

### Building & Testing

* `make build`
    Builds the final, optimized **release** version of the program. The executable will be located at `target/release/moto`. This is also the default command if you just run `make`.

* `make test`
    Runs all unit and integration tests in the project.

### Code Quality & Formatting

* `make fmt`
    Formats all Rust code according to the project's style (using `cargo fmt`).

* `make clippy`
    Runs the Clippy linter to check for common mistakes and un-idiomatic code. This command treats all warnings as errors (`-D warnings`), forcing you to write clean code.

### Documentation & Cleanup

* `make doc`
    Generates all project documentation from `///` comments (including those in `src/grammar.pest`) and automatically opens the result in your web browser.

* `make clean`
    Removes the `target` directory, deleting all build artifacts, cached dependencies, and compiled executables.