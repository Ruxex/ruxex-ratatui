# Ruxex Ratatui Core

[![Crates.io](https://img.shields.io/crates/v/ruxex-ratatui-core)](https://crates.io/crates/ruxex-ratatui-core)
[![Documentation](https://docs.rs/ruxex-ratatui-core/badge.svg)](https://docs.rs/ruxex-ratatui-core)
[![License](https://img.shields.io/crates/l/ruxex-ratatui-core)](../LICENSE)

`ruxex-ratatui-core` provides the core types and traits used by the Ruxex Ratatui terminal UI
library. It is intended for widget libraries, backend integrations, and other code that needs the
core rendering and layout contracts without depending on the complete application-facing crate.

The Rust library name remains `ratatui_core` for source compatibility:

```toml
[dependencies]
ruxex-ratatui-core = "0.1.2"
```

Applications should generally use [`ruxex-ratatui`](https://crates.io/crates/ruxex-ratatui), while
widget and backend authors can depend on this smaller core crate.

This package is maintained as part of the Ruxex fork of [Ratatui](https://github.com/ratatui/ratatui)
and retains the upstream MIT license and attribution.
