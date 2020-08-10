use anyhow::Result;
use hxcmp::matrix;
fn main() -> Result<()> {
    matrix::run()?;
    Ok(())
}
