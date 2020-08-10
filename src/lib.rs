use anyhow::Result;

pub trait ComparisonTokenizer {
    fn name(&self) -> String;
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()>;
    fn parse_tokens_string(&self, input: String) -> Result<()>;
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()>;
}

pub mod data;
pub mod tokenizer;

pub mod matrix;
pub mod tests;

#[cfg(test)]
pub mod tokenizer_test;
