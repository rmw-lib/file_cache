pub struct FileCache {}

impl FileCache {
  pub fn new_with_key_builder<SP, R>(
    num_counters: usize,
    max_cost: i64,
    index: KH,
    spawner: SP,
  ) -> Result<Self, CacheError>
  where
    SP: Fn(BoxFuture<'static, ()>) -> R + Send + Sync + 'static + Copy,
  {
    let max_cost = err::ok!(file_open_limit::get()).unwrap();

    let num_counters = max_cost * 128;
    AsyncCache::<u64, u64, TransparentKeyBuilder<_>>::new_with_key_builder(
      100,
      10,
      TransparentKeyBuilder::<u64>::default(),
      tokio::spawn,
    )
    .unwrap();
  }
}
