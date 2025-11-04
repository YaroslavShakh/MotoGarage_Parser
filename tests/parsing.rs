use anyhow::Result;
use motogarage_parser::{MotogarageParser, Rule};
use pest::Parser;

#[test]
fn test_parse_ident_rule() -> Result<()> {
    let mut pairs = MotogarageParser::parse(Rule::ident, "my_test_identifier_123")?;
    
    let pair = pairs.next().unwrap();
    
    assert_eq!(pair.as_str(), "my_test_identifier_123");
    
    Ok(())
}