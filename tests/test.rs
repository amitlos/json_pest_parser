use json_pest_parser::JsonParser;
use pest::Parser;
use json_pest_parser::Rule;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rule() {
        let input = r#"{"key": "value"}"#;
        assert!(JsonParser::parse(Rule::json, input).is_ok());
    }

    #[test]
    fn test_obj_rule() {
        let input = r#"{"key": "value"}"#;
        assert!(JsonParser::parse(Rule::obj, input).is_ok());

        let empty_obj = r#"{}"#;
        assert!(JsonParser::parse(Rule::obj, empty_obj).is_ok());
    }

    #[test]
    fn test_obj_content_rule() {
        let input = r#""key": "value""#;
        assert!(JsonParser::parse(Rule::obj_content, input).is_ok());

        let multiple_pairs = r#""key1": "value1", "key2": "value2""#;
        assert!(JsonParser::parse(Rule::obj_content, multiple_pairs).is_ok());
    }

    #[test]
    fn test_pair_rule() {
        let input = r#""key": "value""#;
        assert!(JsonParser::parse(Rule::pair, input).is_ok());
    }

    #[test]
    fn test_arr_rule() {
        let input = r#"[1, 2, 3]"#;
        assert!(JsonParser::parse(Rule::arr, input).is_ok());

        let empty_array = r#"[]"#;
        assert!(JsonParser::parse(Rule::arr, empty_array).is_ok());
    }

    #[test]
    fn test_arr_content_rule() {
        let input = r#"1, "string", true, null"#;
        assert!(JsonParser::parse(Rule::arr_content, input).is_ok());
    }

    #[test]
    fn test_val_rule() {
        let inputs = vec![
            r#"{"key": "value"}"#, // Object
            r#"[1, 2, 3]"#,        // Array
            r#""string""#,          // String
            r#"123"#,               // Number
            r#"true"#,              // Boolean
            r#"null"#,              // Null
        ];

        for input in inputs {
            assert!(JsonParser::parse(Rule::val, input).is_ok(), "Failed on input: {}", input);
        }
    }

    #[test]
    fn test_boolean_rule() {
        assert!(JsonParser::parse(Rule::boolean, "true").is_ok());
        assert!(JsonParser::parse(Rule::boolean, "false").is_ok());
    }

    #[test]
    fn test_null_rule() {
        assert!(JsonParser::parse(Rule::null, "null").is_ok());
    }

    #[test]
    fn test_string_rule() {
        let input = r#""string""#;
        assert!(JsonParser::parse(Rule::string, input).is_ok());

        let escaped_string = r#""escaped \"string\"""#;
        assert!(JsonParser::parse(Rule::string, escaped_string).is_ok());
    }

    #[test]
    fn test_inner_rule() {
        let input = r#"inner string without quotes"#;
        assert!(JsonParser::parse(Rule::inner, input).is_ok());
    }

    #[test]
    fn test_num_rule() {
        let inputs = vec![
            "123",           
            "-123",          
            "123.456",       
            "-123.456",      
            "1.23e10",       
            "-1.23e-10",     
        ];

        for input in inputs {
            assert!(JsonParser::parse(Rule::num, input).is_ok(), "Failed on input: {}", input);
        }
    }

    #[test]
    fn test_whitespace_rule() {
        let input = " \t\n\r";
        assert!(JsonParser::parse(Rule::WHITESPACE, input).is_ok());
    }
}
