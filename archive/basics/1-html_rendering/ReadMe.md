# Dependencies

```rust
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
```

- This part builds a simple single-threaded webserver that renders simple html pages.
- 2 Routes : Home & 404

# How to test

```
cargo run
```

http://127.0.0.1:7878