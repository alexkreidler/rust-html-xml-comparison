use anyhow::Result;

fn main() -> Result<()> {
    for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
        println!("{:?}", token?);
    }
    Ok(())
}
