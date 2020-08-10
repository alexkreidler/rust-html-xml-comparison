use anyhow::{anyhow, Context, Error, Result};

use crate::data;

use log::LevelFilter;
use log::{debug, error, info};

fn init() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();
}

fn do_test(name: &'static str, data: &'static str) -> Result<()> {
    init();
    info!("Running test {}", name);
    let tk = crate::parser::get_parsers();
    let mut latest_err: Option<anyhow::Error> = None;
    for tkz in tk {
        let name = tkz.name();
        debug!("Using tokenizer {}", name);
        // TODO: fail fast here by propagating result up, or just skip input
        // TODO: handle panics here
        let out = tkz.parse_tokens_str(data);
        match out {
            Err(e) => {
                error!(
                    "FAILED to parse tokens from input string using tokenizer {}",
                    name
                );
                latest_err = Some(Error::msg(e.to_string()))
            }
            Ok(_) => {}
        };
        // .context(format!(
        //     "Failed to parse tokens from input string using tokenizer {}",
        //     name,
        // ))?;
    }
    info!("Completed test {}", name);
    if latest_err.is_some() {
        Err(latest_err.unwrap())?
    }
    Ok(())
}

#[test]
fn all_tokenizers_basic() -> Result<()> {
    do_test("basic", crate::data::basic)
}

#[test]
fn all_tokenizers_basic2() -> Result<()> {
    do_test("basic2", crate::data::basic2)
}

#[test]
fn all_tokenizers_invalid() -> Result<()> {
    do_test("invalid", data::invalid)
}

#[test]
fn all_tokenizers_invalid2() -> Result<()> {
    do_test("invalid2", data::invalid2)
}

#[test]
fn all_tokenizers_invalid3() -> Result<()> {
    do_test("invalid3", data::invalid3)
}

#[test]
fn all_tokenizers_self_closed() -> Result<()> {
    do_test("self_closed", data::self_closed)
}
