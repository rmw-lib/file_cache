# file_handle_cache

get the file open limit ( windows / mac / linux / freebsd )

use example

```rust
use anyhow::Result;

fn main() -> Result<()> {
    dbg!(file_handle_cache::get()?);
    Ok(())
}
```
