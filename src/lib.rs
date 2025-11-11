#![doc = include_str!("../docs.md")]

use pest::Parser; 
use pest_derive::Parser;
use thiserror::Error; 

// --- 1. PARSER SETUP ---
// This is the main link to pest.
// It tells pest_derive to generate a parser struct named MotogarageParser...
#[derive(Parser)]
#[grammar = "src/grammar.pest"] // ...and to use this grammar file to build it.
pub struct MotogarageParser;

// --- 2. ERROR HANDLING ---
// Defines our library's custom error types.
// 'thiserror' makes it easy to create good error messages.
#[derive(Error, Debug)]
pub enum MotoError {
    // This variant will automatically wrap any parsing errors from pest.
  #[error("Parsing error: {0}")]
  ParseError(#[from] pest::error::Error<Rule>),

    // A custom error for our own logic (e.g., if interpretation fails).
  #[error("Interp error: {0}")]
  InterpreterError(String),
}

// --- 3. ABSTRACT SYNTAX TREE (AST) ---
// These structs and enums represent our language's *structure*.
// The parser's job is to turn text into these Rust types.
// 

// 'Command' is the top-level instruction. A file is a Vec<Command>.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
  Definition(Motorcycle), // Represents a 'DEFINE' command
  Get(Query), // Represents a 'GET' command
  Count(Query), // Represents a 'COUNT' command
}

// Represents the data for a single motorcycle.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Motorcycle {
  pub name: String,
  pub year: Option<u32>, // 'Option' is used because fields are optional
  pub engine_cc: Option<u32>,
  pub bike_type: Option<String>,
}

// Represents a 'GET' or 'COUNT' query.
// It holds an optional condition. If 'None', it means "all bikes".
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
  pub condition: Option<Condition>,
}

// Represents a 'WHERE' clause, like "year > 2020".
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
  pub field: String, // "year"
  pub operator: String, // ">"
  pub value: Value, // Number(2020)
}

// Represents the different types of values our language supports.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Number(u32), // e.g., 2020, 600
  StringType(String), // e.g., sport, cruiser (unquoted identifiers)
  StringLiteral(String), // e.g., "Honda CBR" (quoted strings)
}

// Helper methods to easily extract Rust values from our AST 'Value' enum.
impl Value {
  // Returns the number, or 0 if it's not a number.
  pub fn value_as_number(&self) -> u32 {
    match self {
      Value::Number(n) => *n,
      _ => 0,
    }
  }
  // Returns the string value, or an empty string.
  pub fn value_as_string(&self) -> String {
    match self {
      Value::StringType(s) => s.clone(),
      Value::StringLiteral(s) => s.clone(),
      _ => String::new(),
    }
  }
}

// --- 4. INTERPRETER ---
// The 'Garage' struct holds the state of our program (the list of bikes).
// It executes the AST (the Vec<Command>).
#[derive(Debug, Default)]
pub struct Garage {
  bikes: Vec<Motorcycle>, // Our in-memory "database"
}

impl Garage {
  pub fn new() -> Self {
    Self::default()
  }

    // The main execution loop. It takes the AST and runs each command.
  pub fn execute(&mut self, program: Vec<Command>) -> Result<Vec<String>, MotoError> {
    let mut results = Vec::new(); // Collects output from GET/COUNT

    for command in program {
      match command {
                // If the command is 'Definition', add the bike to our state.
        Command::Definition(bike) => {
          self.bikes.push(bike);
        }
                // If 'Get', run the query and add the list of names to results.
        Command::Get(query) => {
          results.extend(self.run_query_get(query));
        }
                // If 'Count', run the query, count the items, and add the count as a string.
        Command::Count(query) => {
          let count = self.filter_bikes(&query).count();
          results.push(format!("Bikes found: {}", count));
        }
      }
    }
    Ok(results) // Return all collected results.
  }

    // A reusable helper function to filter bikes based on a query.
    // It returns an iterator for efficiency (no new Vec is created here).
  fn filter_bikes<'a>(&'a self, query: &'a Query) -> impl Iterator<Item = &'a Motorcycle> {
    self.bikes
      .iter()
            // .map_or(true, ...) means: if the condition is 'None', return 'true' (match all bikes).
            // Otherwise, call the 'bike.matches(c)' function.
      .filter(move |bike| query.condition.as_ref().map_or(true, |c| bike.matches(c)))
  }

    // The logic for 'GET'. It uses the filter helper and then collects the names.
  fn run_query_get(&self, query: Query) -> Vec<String> {
    self.filter_bikes(&query)
      .map(|bike| bike.name.clone()) // Get only the names
      .collect() // Collect into a new Vec<String>
  }
}

// Logic for checking if a single bike matches a 'WHERE' condition.
impl Motorcycle {
  fn matches(&self, condition: &Condition) -> bool {
    match condition.field.as_str() { // Check which field we are filtering on
      "type" => self
        .bike_type
        .as_ref() // Get an Option<&String>
                    // Check if the bike's type matches the value's string.
        .map_or(false, |t| *t == condition.value.value_as_string()),
      "year" => self.year.map_or(false, |y| { // 'y' is the bike's year
        compare(y, &condition.operator, condition.value.value_as_number())
      }),
      "engine" => self.engine_cc.map_or(false, |e| { // 'e' is the bike's engine
        compare(e, &condition.operator, condition.value.value_as_number())
      }),
      _ => false, // Unknown field, so it's not a match.
    }
  }
}
// A simple comparison helper for numbers.
fn compare(a: u32, op: &str, b: u32) -> bool {
  match op {
    "=" => a == b,
    ">" => a > b,
    "<" => a < b,
    _ => false, // Invalid operator
  }
}
impl Condition {} // This is empty, which is fine.

// --- 5. PARSING LOGIC (Text -> AST) ---
// These functions transform the 'Pairs' from pest into our AST structs.

// The main entry point for parsing.
pub fn parse_moto_file(input: &str) -> Result<Vec<Command>, MotoError> {
    // 1. Call pest to parse the input string using the 'file' rule.
  let pairs = MotogarageParser::parse(Rule::file, input)?; // '?' handles errors
  let mut ast = Vec::new();

    // 2. Iterate over the pairs inside the 'file' rule.
    // We use .next().unwrap().into_inner() to step inside the 'file' pair.
  for pair in pairs.into_iter().next().unwrap().into_inner() {
    match pair.as_rule() {
      Rule::command => ast.push(parse_command(pair)), // Found a command, parse it.
      Rule::EOI => (), // End Of Input, we are done.
      _ => unreachable!(), // Should not happen if grammar is correct.
    }
  }
  Ok(ast) // Return the completed Abstract Syntax Tree (AST).
}

// This function routes a 'command' pair to the correct specific parser.
fn parse_command(pair: pest::iterators::Pair<Rule>) -> Command {
    // A 'command' pair contains either a 'definition', 'query_get', or 'query_count'.
  let inner = pair.into_inner().next().unwrap();
  match inner.as_rule() {
    Rule::definition => Command::Definition(parse_definition(inner)),
    Rule::query_get => Command::Get(parse_query(inner)),
    Rule::query_count => Command::Count(parse_query(inner)),
    _ => unreachable!(),
  }
}

// Parses a 'definition' pair into a 'Motorcycle' struct.
fn parse_definition(pair: pest::iterators::Pair<Rule>) -> Motorcycle {
  let mut inner_pairs = pair.into_inner();
    // The first inner pair is always the 'string_literal' (the name).
  let name = parse_string_literal(inner_pairs.next().unwrap());

  let mut bike = Motorcycle {
    name,
    ..Default::default() // Fill the rest with None/Default
  };

    // The second inner pair is 'properties'. We loop over them.
  let properties_pairs = inner_pairs.next().unwrap().into_inner();
  for prop_pair in properties_pairs { // Each 'prop_pair' is a 'property' rule
    let mut prop_inner = prop_pair.into_inner();
    let field_name = prop_inner.next().unwrap().as_str(); // e.g., "year"
    let value_pair = prop_inner.next().unwrap(); // The 'value' pair

        // Match on the field name and update the bike struct.
    match field_name {
      "year" => bike.year = Some(parse_value(value_pair).value_as_number()),
      "engine" => bike.engine_cc = Some(parse_value(value_pair).value_as_number()),
      "type" => bike.bike_type = Some(parse_value(value_pair).value_as_string()),
      _ => {} // Ignore unknown properties
    }
  }
  bike
}

// Parses a 'query_get' or 'query_count' pair into a 'Query' struct.
// Note: This logic is shared by both GET and COUNT.
fn parse_query(pair: pest::iterators::Pair<Rule>) -> Query {
    // A query pair contains an optional 'where_clause'.
  let where_clause_pair = pair.into_inner().next();

    // If the 'where_clause' exists, get the 'condition' from inside it.
  let condition_pair =
    where_clause_pair.map(|where_pair| where_pair.into_inner().next().unwrap());

    // If the 'condition' exists, parse it.
  let condition = condition_pair.map(parse_condition);

  Query { condition } // Create the Query struct
}

// Parses a 'condition' pair into a 'Condition' struct.
fn parse_condition(pair: pest::iterators::Pair<Rule>) -> Condition {
    // A 'condition' pair contains 'ident', 'operator', 'value'.
  let mut inner = pair.into_inner();
  let field = inner.next().unwrap().as_str().to_string();
  let operator = inner.next().unwrap().as_str().to_string();
  let value = parse_value(inner.next().unwrap());
  Condition {
    field,
    operator,
    value,
  }
}

// Parses a 'value' pair into our 'Value' enum.
fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
    // A 'value' pair contains one of its inner rules.
  let inner = pair.into_inner().next().unwrap();
  match inner.as_rule() {
    Rule::number => Value::Number(inner.as_str().parse().unwrap_or(0)),
    Rule::number_with_unit => {
            // We must strip "cc" before parsing to a number.
      Value::Number(inner.as_str().replace("cc", "").parse().unwrap_or(0))
    }
    Rule::ident => Value::StringType(inner.as_str().to_string()),
    Rule::string_literal => Value::StringLiteral(parse_string_literal(inner)),
    _ => unreachable!(),
  }
}

// Helper to clean up quoted strings.
fn parse_string_literal(pair: pest::iterators::Pair<Rule>) -> String {
  pair.as_str().trim_matches('"').to_string() // Removes the "" from the string.
}