use async_std::{fs::File, path::Path};
use stretto::{AsyncCache, CacheError, ValueRefMut};

#[derive(Clone)]
pub struct FileCache {
  cache: AsyncCache<Box<Path>, File>,
}

impl FileCache {
  pub fn new(open_limit: usize) -> Result<Self, CacheError> {
    let open_limit = open_limit.min(err::ok!(file_open_limit::get()).unwrap() / 2);
    let num_counters = open_limit * 128;

    Ok(Self {
      cache: AsyncCache::new(num_counters, open_limit as _, async_std::task::spawn)?,
    })
  }

  pub async fn get(&mut self, path: impl AsRef<Path>) -> async_std::io::Result<ValueRefMut<File>> {
    let path = Box::from(path.as_ref());
    if let Some(exist) = self.cache.get_mut(&path) {
      return Ok(exist);
    }
    let file = File::open(&path).await?;
    self.cache.insert(path.clone(), file, 1).await;
    err::log!(self.cache.wait().await);
    Ok(self.cache.get_mut(&path).unwrap())
  }
}
