pub struct FileCache {}

impl FileCache {
  pub fn new() -> Self {
    Self {}
  }
}

impl Default for FileCache {
  fn default() -> Self {
    Self::new()
  }
}
