# file_cache

cache file open handle

use example

[./examples/main.rs](./examples/main.rs)

```rust
use anyhow::Result;
use async_std::io::{prelude::SeekExt, ReadExt, SeekFrom};
use file_cache::FileCache;

async fn get(cache: &mut FileCache) -> Result<()> {
  let mut path = std::env::current_exe()?;
  (0..4).for_each(|_| {
    path.pop();
  });
  let path = path.join("Cargo.toml").display().to_string();
  let host = cache.get(path).await?;
  let mut host = host.value();
  host.seek(SeekFrom::Start(0)).await?;
  let mut out = [0u8; 1024];
  let n = host.read(&mut out).await?;
  println!("{}", std::str::from_utf8(&out[..n])?);
  dbg!(n);
  Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
  let mut cache = FileCache::new(2048)?;
  get(&mut cache).await?;
  Ok(())
}
```
