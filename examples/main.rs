use anyhow::Result;

fn main() -> Result<()> {
  let cache = file_handle_cache::FileCache::new(2048);
  Ok(())
}
