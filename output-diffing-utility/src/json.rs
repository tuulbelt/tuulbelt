//! Simple JSON parser and differ with zero dependencies

use std::fmt;

/// A simple JSON value
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

/// JSON parse error
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON parse error: {}", self.message)
    }
}

impl std::error::Error for JsonError {}

/// Parse JSON string into JsonValue
pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    let mut chars = input.trim().chars().peekable();
    parse_value(&mut chars)
}

fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    skip_whitespace(chars);

    match chars.peek() {
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('"') => parse_string(chars).map(JsonValue::String),
        Some('t') | Some('f') => parse_bool(chars),
        Some('n') => parse_null(chars),
        Some('-') | Some('0'..='9') => parse_number(chars),
        _ => Err(JsonError {
            message: "Unexpected character".to_string(),
        }),
    }
}

fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
    while matches!(
        chars.peek(),
        Some(' ') | Some('\n') | Some('\r') | Some('\t')
    ) {
        chars.next();
    }
}

fn parse_object(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    chars.next(); // consume '{'
    let mut pairs = Vec::new();

    skip_whitespace(chars);
    if chars.peek() == Some(&'}') {
        chars.next();
        return Ok(JsonValue::Object(pairs));
    }

    loop {
        skip_whitespace(chars);

        // Parse key
        let key = parse_string(chars)?;

        skip_whitespace(chars);
        if chars.next() != Some(':') {
            return Err(JsonError {
                message: "Expected ':'".to_string(),
            });
        }

        // Parse value
        let value = parse_value(chars)?;
        pairs.push((key, value));

        skip_whitespace(chars);
        match chars.peek() {
            Some(',') => {
                chars.next();
            }
            Some('}') => {
                chars.next();
                break;
            }
            _ => {
                return Err(JsonError {
                    message: "Expected ',' or '}'".to_string(),
                })
            }
        }
    }

    Ok(JsonValue::Object(pairs))
}

fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    chars.next(); // consume '['
    let mut values = Vec::new();

    skip_whitespace(chars);
    if chars.peek() == Some(&']') {
        chars.next();
        return Ok(JsonValue::Array(values));
    }

    loop {
        let value = parse_value(chars)?;
        values.push(value);

        skip_whitespace(chars);
        match chars.peek() {
            Some(',') => {
                chars.next();
            }
            Some(']') => {
                chars.next();
                break;
            }
            _ => {
                return Err(JsonError {
                    message: "Expected ',' or ']'".to_string(),
                })
            }
        }
    }

    Ok(JsonValue::Array(values))
}

fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, JsonError> {
    if chars.next() != Some('"') {
        return Err(JsonError {
            message: "Expected '\"'".to_string(),
        });
    }

    let mut string = String::new();
    loop {
        match chars.next() {
            Some('"') => return Ok(string),
            Some('\\') => match chars.next() {
                Some('n') => string.push('\n'),
                Some('r') => string.push('\r'),
                Some('t') => string.push('\t'),
                Some('\\') => string.push('\\'),
                Some('"') => string.push('"'),
                _ => {
                    return Err(JsonError {
                        message: "Invalid escape sequence".to_string(),
                    })
                }
            },
            Some(c) => string.push(c),
            None => {
                return Err(JsonError {
                    message: "Unterminated string".to_string(),
                })
            }
        }
    }
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    let mut num_str = String::new();

    // Optional minus
    if chars.peek() == Some(&'-') {
        num_str.push(chars.next().unwrap());
    }

    // Digits
    while matches!(
        chars.peek(),
        Some('0'..='9') | Some('.') | Some('e') | Some('E') | Some('+') | Some('-')
    ) {
        num_str.push(chars.next().unwrap());
    }

    num_str
        .parse::<f64>()
        .map(JsonValue::Number)
        .map_err(|_| JsonError {
            message: format!("Invalid number: {}", num_str),
        })
}

fn parse_bool(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    let mut word = String::new();
    while matches!(chars.peek(), Some('a'..='z')) {
        word.push(chars.next().unwrap());
    }

    match word.as_str() {
        "true" => Ok(JsonValue::Bool(true)),
        "false" => Ok(JsonValue::Bool(false)),
        _ => Err(JsonError {
            message: format!("Invalid boolean: {}", word),
        }),
    }
}

fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<JsonValue, JsonError> {
    let mut word = String::new();
    while matches!(chars.peek(), Some('a'..='z')) {
        word.push(chars.next().unwrap());
    }

    if word == "null" {
        Ok(JsonValue::Null)
    } else {
        Err(JsonError {
            message: format!("Invalid null: {}", word),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let result = parse_json("null").unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_json("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(parse_json("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_json("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(parse_json("-2.5").unwrap(), JsonValue::Number(-2.5));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_json(r#""hello""#).unwrap(),
            JsonValue::String("hello".to_string())
        );
        assert_eq!(
            parse_json(r#""hello\nworld""#).unwrap(),
            JsonValue::String("hello\nworld".to_string())
        );
    }

    #[test]
    fn test_parse_array() {
        let result = parse_json("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_object() {
        let result = parse_json(r#"{"name": "Alice", "age": 30}"#).unwrap();
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("name".to_string(), JsonValue::String("Alice".to_string())),
                ("age".to_string(), JsonValue::Number(30.0)),
            ])
        );
    }

    #[test]
    fn test_parse_nested() {
        let result = parse_json(r#"{"users": [{"name": "Alice"}]}"#).unwrap();
        match result {
            JsonValue::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "users");
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_empty_object() {
        let result = parse_json("{}").unwrap();
        assert_eq!(result, JsonValue::Object(vec![]));
    }

    #[test]
    fn test_parse_empty_array() {
        let result = parse_json("[]").unwrap();
        assert_eq!(result, JsonValue::Array(vec![]));
    }

    #[test]
    fn test_parse_whitespace() {
        let result = parse_json("  {  \"key\"  :  \"value\"  }  ").unwrap();
        assert_eq!(
            result,
            JsonValue::Object(vec![(
                "key".to_string(),
                JsonValue::String("value".to_string())
            )])
        );
    }
}
