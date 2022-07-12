use anyhow::Result;
use file_cache::FileCache;

async fn get(cache: &mut FileCache<'_>) -> Result<()> {
  let path = "/Users/z/rmw/file_handle_cache/1.txt";
  let host = cache.get(path.as_bytes()).await?;
  dbg!(host);
  Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
  let mut cache = FileCache::new(2048)?;
  get(&mut cache).await?;
  Ok(())
}
