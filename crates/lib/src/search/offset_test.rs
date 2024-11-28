use crate::search::offset::parse_from_offset;

#[test]
fn test_parse_from_offset() {
    assert!(parse_from_offset(r#"from "2024-05-28T17:55:08.145+02:00""#).is_ok());
}

#[test]
fn test_parse_from_end_minus_number() {
    assert!(parse_from_offset(r#"from end - 10"#).is_ok());
}
