# Living Ordinal Playground

This crate glues the simulated watcher, mirror bridge, and viewer API together.

## Getting started

```bash
cd ord2
cargo run --bin watcher_daemon
```

In a second terminal, run:

```bash
cd ord2
cargo run --bin viewer_api
```

With the API running you can explore records from the CLI or the React UI:

```bash
cd ord2
cargo run --bin viewer list
```

The React application lives in `../ord2-viewer`.
