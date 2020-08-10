use anyhow::Result;

use crate::ComparisonTokenizer;
use log::debug;
pub struct XMLParser {}
impl ComparisonTokenizer for XMLParser {
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        // After testing, there's no difference in out examples between `from` and `from_fragment`
        let tk = xmlparser::Tokenizer::from_fragment(
            input,
            std::ops::Range {
                start: 0,
                end: input.len(),
            },
        );
        let mut count = 0;
        for out in tk {
            let _token = out?;
            count += 1;
        }
        debug!("total xmlparser events {}", count);
        Ok(())
    }
    fn name(&self) -> String {
        "xml-parser".to_string()
    }
    fn parse_tokens_string(&self, input: String) -> Result<()> {
        let tk = xmlparser::Tokenizer::from(input.as_str());
        let mut count = 0;
        for out in tk {
            let _token = out?;
            count += 1;
        }
        debug!("total xmlparser events {}", count);
        Ok(())
    }
    fn parse_tokens_reader(&self, input: &mut dyn std::io::BufRead) -> Result<()> {
        let mut mem = String::new();
        input.read_to_string(&mut mem)?;
        let tk = xmlparser::Tokenizer::from(mem.as_str());
        let mut count = 0;
        for out in tk {
            let _token = out?;
            count += 1;
        }
        debug!("total xmlparser events {}", count);
        Ok(())
    }
}

pub struct QuickXML {}
use quick_xml::events::Event;
use quick_xml::Reader;

impl ComparisonTokenizer for QuickXML {
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        let mut reader = Reader::from_str(input);
        reader.trim_text(true);

        let mut count = 0;
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ))
                }
                _ => {
                    count += 1;
                }
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        debug!("total quick-xml events {}", count);
        Ok(())
    }

    fn name(&self) -> String {
        "quick-xml".to_string()
    }
    fn parse_tokens_string(&self, input: String) -> Result<()> {
        let mut reader = Reader::from_str(input.as_str());
        reader.trim_text(true);

        let mut count = 0;
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ))
                }
                _ => {
                    count += 1;
                }
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        debug!("total quick-xml events {}", count);
        Ok(())
    }
    fn parse_tokens_reader(&self, input: &mut dyn std::io::BufRead) -> Result<()> {
        let mut reader = Reader::from_reader(input);
        reader.trim_text(true);

        let mut count = 0;
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ))
                }
                _ => {
                    count += 1;
                }
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        debug!("total quick-xml events {}", count);
        Ok(())
    }
}

pub struct XmlRs {}

use xml::reader::EventReader;

impl ComparisonTokenizer for XmlRs {
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        let parser = EventReader::new(input.as_bytes());
        let mut count = 0;
        for e in parser {
            let _token = e?;
            count += 1;
        }
        debug!("total xml-rs events {}", count);
        Ok(())
    }

    fn name(&self) -> String {
        "xml-rs".to_string()
    }
    fn parse_tokens_string(&self, input: String) -> Result<()> {
        let parser = EventReader::new(input.as_bytes());
        let mut count = 0;
        for e in parser {
            let _token = e?;
            count += 1;
        }
        debug!("total xml-rs events {}", count);
        Ok(())
    }
    fn parse_tokens_reader(&self, input: &mut dyn std::io::BufRead) -> Result<()> {
        let parser = EventReader::new(input);
        let mut count = 0;
        for e in parser {
            let _token = e?;
            count += 1;
        }
        debug!("total xml-rs events {}", count);
        Ok(())
    }
}

pub type Tokenizers = Vec<Box<dyn ComparisonTokenizer>>;

pub fn get_tokenizers() -> Tokenizers {
    let t: Tokenizers = vec![
        Box::new(XmlRs {}),
        Box::new(XMLParser {}),
        Box::new(QuickXML {}),
    ];
    t
}
