# motogarage_parser

**Project Name:** motogarage_parser
**Description:** A parser and interpreter for MotoGarage DSL, a language for managing motorcycle collections.

---
Зараз в репо тільки обов'язкова частина для здачі опису парсера, через пару днів зроблю більше.
## Plan
This project implements a parser for a custom Domain-Specific Language (DSL) for managing a "garage" of motorcycles.

### 1. Граматика

The parser is designed to process text files (e.g. `.moto`),that contain a set of commands. In the final version, the language will support:

* Defining new bikes (`DEFINE bike ... { ... }`)
* Querying the collection (`GET BIKES WHERE ...`)

when i add this two, i will think about adding some more commands, cause two is not enough i think

At the current, initial stage, the grammar (`src/grammar.pest`) defines only basic "atoms," such as  **identifiers** (`ident`).

### 2. Процес

1.  **Pest Parser:** I use the `pest` library to define our grammar in PEG (Parsing Expression Grammars) format in the `src/grammar.pest` file.
2.  **Parser Generation:** `pest_derive` automatically generates the parser (`MotogarageParser` in `src/lib.rs`) based on this grammar.
3.  **AST (Abstract Syntax Tree):** (To be added later) The generated "tree of pairs" (Pairs) will be transformed into strongly-typed Rust structs (e.g., `Command`, `Motorcycle`)hat form the AST.

### 3. How the results are used

(To be added later) After the input text is converted into an AST, an "interpreter" (e.g., a Garage struct) will traverse this tree and execute the commands:

* `DEFINE` will add new Motorcycle objects to an internal collection.
* `GET` will filter this collection and return the results.