use async_std::io::File;
use stretto::{AsyncCache, CacheError, TransparentKeyBuilder};

pub struct FileCache {}

impl FileCache {
  pub fn new() -> Result<Self, CacheError> {
    let open_limit = err::ok!(file_open_limit::get()).unwrap() / 2;
    let num_counters = open_limit * 128;
    AsyncCache::new(num_counters, open_limit as _, async_std::task::spawn).unwrap();
  }
}
