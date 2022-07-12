# file_cache

file open handle cache

use example

```rust
use anyhow::Result;

fn main() -> Result<()> {
    dbg!(file_handle_cache::get()?);
    Ok(())
}
```
