use anyhow::Result;
use file_limit;

fn main() -> Result<()> {
    dbg!(file_limit::get()?);
    Ok(())
}
