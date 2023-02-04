# Basics of a Rust application for a webserver WITH SurrealDB.

- simulates a simple database and SQL requests.

# Global dependencies

- Cargo Watch

```
$ cargo install cargo-watch
```

# Dependencies

```rust
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};
```

# How to run

$ cargo watch -q -c -x "run -q"