use anyhow::Result;

use crate::Parser;
use log::debug;
pub struct XMLParser {}
impl Parser for XMLParser {
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
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()> {
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

impl Parser for QuickXML {
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
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()> {
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

impl Parser for XmlRs {
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
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()> {
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

pub struct Html5Ever {}

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

impl Parser for Html5Ever {
    fn name(&self) -> String {
        "html5ever".to_string()
    }
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut inp = input.as_bytes();

        let _dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut inp)?;

        Ok(())
    }
    fn parse_tokens_string(&self, input: String) -> Result<()> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut inp = input.as_bytes();

        let _dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut inp)?;

        Ok(())
    }
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // let mut inp = input;
        let _dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(Box::new(input).as_mut())?;

        Ok(())
    }
}

pub struct HtmlParser {}
use html_parser::Dom;

// TODO: wrap all tests in panic catch_unwind at the call site, rather than just on this code
impl Parser for HtmlParser {
    fn name(&self) -> String {
        "html-parser".to_string()
    }
    fn parse_tokens_str<'a>(&self, input: &'a str) -> Result<()> {
        let result = std::panic::catch_unwind(|| Dom::parse(input));
        match result {
            Err(_) => Err(anyhow::anyhow!("unexpected panic from parser")),
            Ok(r) => {
                r?;
                Ok(())
            }
        }
    }
    fn parse_tokens_string(&self, input: String) -> Result<()> {
        let result = std::panic::catch_unwind(|| Dom::parse(input.as_str()));
        match result {
            Err(_) => Err(anyhow::anyhow!("unexpected panic from parser")),
            Ok(r) => {
                r?;
                Ok(())
            }
        }
    }
    fn parse_tokens_reader<'a>(&self, input: &'a mut dyn std::io::BufRead) -> Result<()> {
        let mut intermediate = String::new();
        input.read_to_string(&mut intermediate)?;
        let result = std::panic::catch_unwind(|| Dom::parse(intermediate.as_str()));
        match result {
            Err(_) => Err(anyhow::anyhow!("unexpected panic from parser")),
            Ok(r) => {
                r?;
                Ok(())
            }
        }
    }
}

pub type Parsers = Vec<Box<dyn Parser>>;

pub fn get_parsers() -> Parsers {
    let t: Parsers = vec![
        Box::new(XmlRs {}),
        Box::new(XMLParser {}),
        Box::new(QuickXML {}),
        Box::new(Html5Ever {}),
        Box::new(HtmlParser {}),
    ];
    t
}
