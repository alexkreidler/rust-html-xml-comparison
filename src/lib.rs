use anyhow::Result;

pub trait ComparisonTokenizer {
    fn name(&self) -> String;
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()>;
    fn parse_tokens_string(&self, input: String) -> Result<()>;
    fn parse_tokens_reader(&self, input: &mut dyn std::io::BufRead) -> Result<()>;
}

pub mod data;
pub mod tokenizer;

pub mod matrix;

#[cfg(test)]
pub mod tokenizer_test;
