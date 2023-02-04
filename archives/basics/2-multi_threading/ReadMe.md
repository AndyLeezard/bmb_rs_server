# New Dependencies

```rust
use std::thread;
use std::time::Duration;
```

# Challenges

- The server was previously single-threaded; it can jeopardize the UX as it scales; one heavy request can result in an unnecessary waiting queue of other light requests. Therefore in this commit thread pools are implemented to handle theses scenarios.

# Thread pools

- Start with the public API.
- Spawning a thread for every single connection is not a smart approach to this matter as the server becomes vulnerable to being exhausted.

# How to test

- Copy and paste the codes from `./single_threaded.rs` to `src/main.rs`.

```
cargo run
```

- Open two seperate browsers you can observe at the same time.
- Connect to `http://127.0.0.1:7878/sleep` on one side (**A**), and `http://127.0.0.1:7878` on the other side (**B**). On the **B** browser, normally you should be able to see the html rendered immediately, but it takes as long as the other one with the "heavy request" (**A**), because the server is processing the previous request from **A** before handling the latest request **B**.