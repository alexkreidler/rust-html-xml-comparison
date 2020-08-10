use anyhow::Result;
use tabwriter::TabWriter;

pub struct Test {
    name: &'static str,
    input: Input,
}

pub enum Input {
    Static(&'static str),
}

type Tests = Vec<Test>;

use crate::data;

pub fn get_tests() -> Tests {
    return vec![
        Test {
            name: "basic",
            input: Input::Static(data::basic),
        },
        Test {
            name: "basic2",
            input: Input::Static(data::basic2),
        },
        Test {
            name: "invalid",
            input: Input::Static(data::invalid),
        },
        Test {
            name: "invalid2",
            input: Input::Static(data::invalid2),
        },
        Test {
            name: "invalid3",
            input: Input::Static(data::invalid3),
        },
        Test {
            name: "self_closed",
            input: Input::Static(data::self_closed),
        },
    ];
}

use log::error;

fn do_test(test: &Test, tkz: &Box<dyn crate::ComparisonTokenizer>) -> Result<String> {
    let out = match test.input {
        Input::Static(s) => tkz.parse_tokens_str(s),
    };
    Ok(match out {
        Err(e) => {
            error!(
                "FAILED to parse tokens from input string using tokenizer {}",
                tkz.name()
            );
            "FAILED".to_string()
        }
        Ok(_) => "SUCCEEDED".to_string(),
    })
}

use std::io::Write;

pub fn run() -> Result<()> {
    println!("Generating test matrix");

    let mut writer = TabWriter::new(std::io::stdout()).padding(8);

    let tk = crate::tokenizer::get_tokenizers();

    let ts: Vec<String> = tk.iter().map(|t| t.name()).collect();

    println!("{:?}", ts);
    let w = &mut writer;
    w.write(format!("{}{}{}", "Test Name\t", ts.join("\t"), "\n").as_bytes())?;

    // TODO: maybe keep track of totals for each person
    for test in get_tests() {
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
