use anyhow::Result;

use crate::data;

use log::LevelFilter;
use log::{debug, info};

fn init() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();
}

fn do_test(name: &'static str, data: &'static str) -> Result<()> {
    init();
    info!("Running test {}", name);
    let tk = crate::tokenizer::get_tokenizers();
    for tkz in tk {
        debug!("Using tokenizer {}", tkz.name());
        tkz.parse_tokens_str(data)?;
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
fn all_tokenizers_self_closed() -> Result<()> {
    do_test("self_closed", data::self_closed)
}
