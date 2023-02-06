# Feature

- Realtime chat

# Dependencies

```rust
rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
rocket = { version = "0.5.0-rc.2", features = ["json"] }
```

# Drawbacks

- no db persistance yet.

# Customize Frontend?

- URL to source code git repo (Vite + React + TS + SWC) [https://github.com/AndyLeezard/vite-react-swc-ts-rustchat]

# Disclaimer

- The source code added CORS options on top of this [example](https://github.com/SergioBenitez/Rocket/tree/master/examples)