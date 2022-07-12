use anyhow::Result;

fn main() -> Result<()> {
  dbg!(file_open_limit::get()?);
  Ok(())
}
