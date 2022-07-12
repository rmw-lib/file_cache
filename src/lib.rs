use async_std::fs::File;
use stretto::{AsyncCache, CacheError, TransparentKeyBuilder};

pub struct FileCache {
  cache: u8,
}

impl FileCache {
  pub fn new() -> Result<Self, CacheError> {
    let open_limit = err::ok!(file_open_limit::get()).unwrap() / 2;
    let num_counters = open_limit * 128;
    Ok(Self {
      cache: AsyncCache::new(num_counters, open_limit as _, async_std::task::spawn)?,
    })
  }
}
