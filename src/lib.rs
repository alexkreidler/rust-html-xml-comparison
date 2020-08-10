use anyhow::Result;

pub trait Parser {
    fn name(&self) -> String;
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()>;
    fn parse_tokens_string(&self, input: String) -> Result<()>;

    // This mutable reference results in unwind unsafe code
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()>;
}

pub mod data;
pub mod parser;

pub mod matrix;
pub mod tests;

#[cfg(test)]
pub mod parser_test;
