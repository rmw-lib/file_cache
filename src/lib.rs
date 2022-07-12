use async_std::fs::File;
use stretto::{AsyncCache, CacheError, TransparentKeyBuilder};

#[derive(Clone)]
pub struct FileCache {
  cache: AsyncCache<Box<[u8]>, File>,
}

impl FileCache {
  pub fn new(open_limit: usize) -> Result<Self, CacheError> {
    let open_limit = open_limit.min(err::ok!(file_open_limit::get()).unwrap() / 2);
    let num_counters = open_limit * 128;

    Ok(Self {
      cache: AsyncCache::new(num_counters, open_limit as _, async_std::task::spawn)?,
    })
  }
}
