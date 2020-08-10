use anyhow::Result;

use crate::ComparisonTokenizer;
pub struct XMLParser {}
impl ComparisonTokenizer for XMLParser {
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        let tk = xmlparser::Tokenizer::from(input);
        let mut count = 0;
        for out in tk {
            let _token = out?;
            count += 1;
        }
        println!("total xmlparser events {}", count);
        Ok(())
    }
    fn name(&self) -> String {
        "xml-parser".to_string()
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
        println!("total quick-xml events {}", count);
        Ok(())
    }

    fn name(&self) -> String {
        "quick-xml".to_string()
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
        println!("total xml-rs events {}", count);
        Ok(())
    }

    fn name(&self) -> String {
        "xml-rs".to_string()
    }
}

pub fn get_tokenizers() -> Vec<Box<dyn ComparisonTokenizer>> {
    let t: Vec<Box<dyn ComparisonTokenizer>> = vec![
        Box::new(XMLParser {}),
        Box::new(QuickXML {}),
        Box::new(XmlRs {}),
    ];
    t
}
