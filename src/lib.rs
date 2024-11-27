use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// json parser using grammar defined in grammar.pest file
#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct JsonParser;

pub enum JsonContent<'a> {
    Obj,
    Arr(Vec<JsonContent<'a>>),
    Str(&'a str),
    Num(f64),
    Bool(bool),
    Null,
}

pub fn json_content_to_Str(val: &JsonContent) -> String {
    use JsonContent::*;

    match val {
        Obj => {
            String::from("Object")
        }
        Arr(a) => {
            let contents: Vec<_> = a.iter().map(json_content_to_Str).collect();
            format!("[{}]", contents.join(","))
        }
        Str(s) => format!("\"{}\"", s),
        Num(n) => format!("{}", n),
        Bool(b) => format!("{}", b),
        Null => format!("null"),
    }
}

#[derive(Error, Debug)]
pub enum JSONParsingError {
    /// Error from pest library which current error type inherits
    #[error("An error during parsing occured: {pest_error}")]
    ParseError {
        #[from]
        pest_error: pest::error::Error<Rule>,
    },
    /// If input file is empty the error occurs with corresponding message
    #[error("Parsed .json file is empty.")]
    EmptyFile,
}


pub fn parse_json_str(input_str : &str) -> Result<JsonContent, JSONParsingError>
{
    if input_str.trim().is_empty() {
        return Err(JSONParsingError::EmptyFile);
    }

    let json_content = JsonParser::parse(Rule::json, input_str)?.next().unwrap();

    Ok(pair_to_json_content(json_content))
}

fn pair_to_json_content(pair: pest::iterators::Pair<Rule>) -> JsonContent 
{
    match pair.as_rule() {
        Rule::obj | Rule::obj_content => JsonContent::Obj,
        Rule::arr | Rule::arr_content => JsonContent::Arr(pair.into_inner().map(pair_to_json_content).collect()),
        Rule::string => JsonContent::Str(pair.into_inner().next().unwrap().as_str()),
        Rule::num => JsonContent::Num(pair.as_str().parse().unwrap()),
        Rule::boolean => JsonContent::Bool(pair.as_str().parse().unwrap()),
        Rule::null => JsonContent::Null,
        Rule::json
        | Rule::inner
        | Rule::EOI
        | Rule::pair
        | Rule::val
        | Rule::WHITESPACE => unreachable!(),
    }
}





