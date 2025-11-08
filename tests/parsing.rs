use anyhow::Result;
use assert_matches::assert_matches;
use motogarage_parser::Command;
use motogarage_parser::{
    Condition, MotogarageParser, Motorcycle, Query, Rule, Value, parse_moto_file,
};
use pest::Parser;

// Test for rule: line_comment (also tests WHITESPACE)
#[test]
fn test_parse_comment() -> Result<()> {
    let input = r#"
        // This is a comment
        DEFINE bike "Test Bike" { year: 2000 }
        // Another comment
    "#;
    let ast = parse_moto_file(input)?;
    assert_eq!(ast.len(), 1);
    assert_matches!(ast[0], Command::Definition(_));
    Ok(())
}

// Test for rules: ident, number, number_with_unit, string_literal, value
#[test]
fn test_parse_atoms() -> Result<()> {
    let pairs = MotogarageParser::parse(Rule::ident, "my_field")?;
    assert_eq!(pairs.as_str(), "my_field");
    let pairs = MotogarageParser::parse(Rule::number, "12345")?;
    assert_eq!(pairs.as_str(), "12345");
    let pairs = MotogarageParser::parse(Rule::number_with_unit, "600cc")?;
    assert_eq!(pairs.as_str(), "600cc");
    let pairs = MotogarageParser::parse(Rule::string_literal, r#""Hello World""#)?;
    assert_eq!(pairs.as_str(), r#""Hello World""#);

    Ok(())
}

// Test for rule: property
#[test]
fn test_parse_property() -> Result<()> {
    let mut pairs = MotogarageParser::parse(Rule::property, "year: 2020")?;
    let property_pair = pairs.next().unwrap();
    let mut inner = property_pair.into_inner();

    assert_eq!(inner.next().unwrap().as_str(), "year");
    assert_eq!(inner.next().unwrap().as_str(), "2020");
    Ok(())
}

// Test for rule: definition
#[test]
fn test_parse_definition() -> Result<()> {
    let input = r#"
        DEFINE bike "Honda CBR600RR" {
            year: 2021,
            engine: 599cc,
            type: sport
        }
    "#;
    let ast = parse_moto_file(input)?;
    assert_eq!(ast.len(), 1);
    assert_matches!(&ast[0], Command::Definition(bike) if
        bike == &Motorcycle {
            name: "Honda CBR600RR".to_string(),
            year: Some(2021),
            engine_cc: Some(599),
            bike_type: Some("sport".to_string()),
        }
    );
    Ok(())
}

// Test for rules: query_get, where_clause, condition, operator
#[test]
fn test_parse_query_get() -> Result<()> {
    let input = "GET BIKES WHERE type = sport";
    let ast = parse_moto_file(input)?;
    assert_eq!(ast.len(), 1);
    assert_matches!(&ast[0], Command::Get(query) if
        query == &Query {
            condition: Some(Condition {
                field: "type".to_string(),
                operator: "=".to_string(),
                value: Value::StringType("sport".to_string()),
            })
        }
    );
    Ok(())
}

// Test for rule: query_count
#[test]
fn test_parse_query_count() -> Result<()> {
    let input = "COUNT BIKES WHERE year > 2020";
    let ast = parse_moto_file(input)?;
    assert_eq!(ast.len(), 1);
    assert_matches!(&ast[0], Command::Count(query) if
        query == &Query {
            condition: Some(Condition {
                field: "year".to_string(),
                operator: ">".to_string(),
                value: Value::Number(2020),
            })
        }
    );
    Ok(())
}

// Test for rule: file (comprehensive test)
#[test]
fn test_parse_full_file() -> Result<()> {
    let input = r#"
        DEFINE bike "Bike 1" { year: 2010 }
        
        // Comment
        GET BIKES WHERE year > 2000

        DEFINE bike "Bike 2" { type: cruiser }

        COUNT BIKES
    "#;
    let ast = parse_moto_file(input)?;
    assert_eq!(ast.len(), 4);
    assert_matches!(&ast[0], Command::Definition(_));
    assert_matches!(&ast[1], Command::Get(_));
    assert_matches!(&ast[2], Command::Definition(_));
    assert_matches!(&ast[3], Command::Count(_));
    Ok(())
}

// Test for parsing error
#[test]
fn test_parse_error() {
    let input = "DEFINE bike { year: 2000 }"; // Missing name
    let result = parse_moto_file(input);
    assert!(result.is_err());
    assert_matches!(
        result.unwrap_err(),
        motogarage_parser::MotoError::ParseError(_)
    );
}
