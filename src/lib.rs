use std::hash::{Hash, Hasher};

use ahash::AHasher;
use async_std::file::File;
use stretto::{AsyncCache, CacheError, TransparentKeyBuilder};

#[derive(Debug, Default)]
struct Hasher(AHasher);

impl<K: Hash + Eq + ?Sized> stretto::KeyBuilder<K> for Hasher {
  fn hash_index(&self, key: &K) -> u64 {
    let mut hasher = AHasher::default();
    key.hash(hasher);
    hasher.finish()
  }
}

pub struct FileCache {}

impl FileCache {
  pub fn new() -> Result<Self, CacheError> {
    let open_limit = err::ok!(file_open_limit::get()).unwrap() / 2;

    let num_counters = open_limit * 128;
    AsyncCache::new_with_key_builder(
      num_counters,
      open_limit,
      Hasher::default(),
      async_std::task::spawn,
    )
    .unwrap();
  }
}
