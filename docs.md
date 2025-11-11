# motogarage_parser Documentation

A simple parser and interpreter crate for the MotoGarage DSL.

## Overview

The **motogarage_parser** is a parser that processes a custom Domain-Specific Language (DSL) for managing motorcycle collections. It uses a **Pest** grammar to parse text files.

It extracts definitions (like `DEFINE bike ...`) and queries (like `GET BIKES` or `COUNT BIKES`) and represents them as an Abstract Syntax Tree (AST). An included interpreter (`Garage`) can then execute this AST to manage an in-memory collection and return query results.

## Features

* Parses `DEFINE bike "Name" { ... }` definitions.
* Parses `GET BIKES` and `COUNT BIKES` queries.
* Supports `WHERE` clauses with operators (`=`, `>`, `<`).
* Handles and ignores whitespace and `//` style comments.
* Provides a clean AST representation (e.g., `Command`, `Motorcycle`, `Query`).
* Includes an interpreter (`Garage`) to execute commands and return results.

## Example

```rust
use motogarage_parser::{parse_moto_file, Garage};

let source = r#"
// Define a new bike
DEFINE bike "Honda CBR600RR" {
    year: 2021,
    type: sport
}

// Query for sport bikes
GET BIKES WHERE type = sport
"#;

// 1. Parse the file into an AST
let ast = parse_moto_file(source).unwrap();

// 2. Create an interpreter and execute the AST
let mut garage = Garage::new();
let results = garage.execute(ast).unwrap();

// results will be: vec!["Honda CBR600RR"]

## Public API Summary

### `parse_moto_file(input: &str)`

Parses a `.moto` source string and returns a `Result<Vec<Command>, MotoError>`, which is the program's Abstract Syntax Tree (AST).

### `Garage`

Represents the interpreter, which holds the state (the collection of bikes).

* `new() -> Self`: Creates a new, empty `Garage`.
* `execute(&mut self, program: Vec<Command>) -> Result<Vec<String>, MotoError>`: Executes the AST and returns a `Vec<String>` containing the results from `GET` or `COUNT` queries.

### `Command` (AST Node)

Represents a single top-level command in the AST.

* `Definition(Motorcycle)`
* `Get(Query)`
* `Count(Query)`

### `MotoError` (Error Type)

The enum representing all possible parsing or interpretation errors.

* `ParseError(pest::error::Error<Rule>)`
* `InterpreterError(String)`

## CLI Usage

The crate also includes a binary CLI application:

# Parse and execute a file
moto parse my_garage1.moto

# Show credits
moto credits