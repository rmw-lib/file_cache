# file_open_limit

get the file open limit ( windows / mac / linux / freebsd )

use example

```rust
use anyhow::Result;

fn main() -> Result<()> {
    dbg!(file_open_limit::get()?);
    Ok(())
}
```
