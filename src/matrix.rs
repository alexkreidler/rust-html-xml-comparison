use anyhow::{anyhow, Result};
use tabwriter::TabWriter;

use crate::tests::*;

use log::error;
// use std::borrow::BorrowMut;
// use std::ops::Deref;

fn do_test(test: &Test, tkz: &Box<dyn crate::Parser>) -> Result<String> {
    let out = match &test.input {
        Input::Static(s) => tkz.parse_tokens_str(s),
        Input::AString(s) => tkz.parse_tokens_string(s.clone()),
        Input::Reader(r) => Err(anyhow!("unsupported for now, ownership issues")), //tkz.parse_tokens_reader(r.deref()),
    };
    Ok(match out {
        Err(e) => {
            error!(
                "FAILED to parse tokens from input string using tokenizer {}, {}",
                tkz.name(),
                e
            );
            "FAILED".to_string()
        }
        Ok(_) => "SUCCEEDED".to_string(),
    })
}

use std::io::Write;

pub fn run_matrix<W: std::io::Write>(w: W, tests: Tests) -> Result<()> {
    let mut writer = TabWriter::new(w).padding(8);

    let tk = crate::parser::get_parsers();

    let ts: Vec<String> = tk.iter().map(|t| t.name()).collect();

    let w = &mut writer;
    w.write(format!("{}{}{}", "Test Name\t", ts.join("\t"), "\n").as_bytes())?;

    // TODO: maybe keep track of totals for each person
    for test in tests {
        w.write(format!("{}{}", test.name, "\t").as_bytes())?;
        for tkz in &tk {
            let result = do_test(&test, tkz)?;
            w.write(format!("{}{}", result, "\t").as_bytes())?;
        }
        w.write("\n".as_bytes())?;
    }
    w.flush()?;

    Ok(())
}
