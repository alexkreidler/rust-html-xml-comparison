use anyhow::Result;

pub trait ComparisonTokenizer {
    fn name(&self) -> String;
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()>;
}

pub mod data;
pub mod tokenizer;

#[cfg(test)]
pub mod tokenizer_test;
