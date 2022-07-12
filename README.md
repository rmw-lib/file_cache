# file_limit

get the file open limit ( windows / mac / linux / freebsd )

use example

```rust
use anyhow::Result;
use file_limit;

fn main() -> Result<()> {
    dbg!(file_limit::get()?);
    Ok(())
}
```
