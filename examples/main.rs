use anyhow::Result;

fn main() -> Result<()> {
  dbg!(file_handle_cache::get()?);
  Ok(())
}
