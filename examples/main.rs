use anyhow::Result;
use async_std::io::{prelude::SeekExt, ReadExt, SeekFrom};
use file_cache::FileCache;

async fn get(cache: &mut FileCache<'_>) -> Result<()> {
  let path = "/Users/z/rmw/file_handle_cache/1.txt";
  let host = cache.get(path.as_bytes()).await?;
  let mut host = host.value();
  host.seek(SeekFrom::Start(0)).await?;
  let mut out = [0u8; 1024];
  let n = host.read(&mut out).await?;
  dbg!(std::str::from_utf8(&out[..n])?);
  Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
  let mut cache = FileCache::new(2048)?;
  get(&mut cache).await?;
  Ok(())
}
