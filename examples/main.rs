use anyhow::Result;
use file_handle_cache::FileCache;

#[async_std::main]
async fn main() -> Result<()> {
  let mut cache = file_handle_cache::FileCache::new(2048)?;
  let path = "/Users/z/rmw/file_handle_cache/1.txt";
  let host = cache.get(path.as_bytes()).await?;
  dbg!(host);
  Ok(())
}
