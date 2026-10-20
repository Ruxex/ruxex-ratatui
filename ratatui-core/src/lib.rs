#![no_std]
// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
//! **ruxex-ratatui-core** is the core library used by the [Ruxex Ratatui] project,
//! providing the essential building blocks for creating rich terminal user interfaces in Rust.
//!
//! [Ruxex Ratatui]: https://github.com/Ruxex/ruxex-ratatui
//!
//! # Why `ruxex-ratatui-core`?
//!
//! The `ruxex-ratatui-core` crate is split from the main [`ruxex-ratatui`](https://crates.io/crates/ruxex-ratatui)
//! crate to offer better stability for widget library authors and advanced integrations. Widget
//! libraries should generally depend on `ruxex-ratatui-core`, benefiting from a stable API and reducing
//! the need for frequent updates.
//!
//! Most applications, on the other hand, should depend on the main `ruxex-ratatui` crate, which
//! includes built-in widgets, backend re-exports, and higher-level setup helpers.
//!
//! In practice:
//!
//! - Use [`ruxex-ratatui`] to build applications.
//! - Use `ruxex-ratatui-core` to implement widgets, backend integrations, or other code that needs the
//!   core rendering and layout contracts directly.
//!
//! # Installation
//!
//! Add `ruxex-ratatui-core` to your `Cargo.toml`:
//!
//! ```shell
//! cargo add ruxex-ratatui-core
//! ```
//!
//! # Crate Organization
//!
//! `ruxex-ratatui-core` is part of the Ruxex Ratatui workspace that was modularized in version 0.30.0 to
//! improve compilation times, API stability, and dependency management. This crate provides the
//! foundational types and traits that other crates in the workspace depend on.
//!
//! **When to use `ruxex-ratatui-core`:**
//!
//! - Building widget libraries that implement [`Widget`] or [`StatefulWidget`]
//! - Building custom integrations on top of Ratatui's core rendering contracts
//! - You want minimal dependencies and faster compilation times
//! - You need maximum API stability (core types change less frequently)
//!
//! **When to use the main [`ruxex-ratatui`] crate:**
//!
//! - Building applications
//! - You want built-in widgets, backend re-exports, and setup helpers such as `ratatui::run`
//!
//! For detailed information about the workspace organization, see [ARCHITECTURE.md].
//!
//! [`ruxex-ratatui`]: https://crates.io/crates/ruxex-ratatui
//! [`Widget`]: widgets::Widget
//! [`StatefulWidget`]: widgets::StatefulWidget
//! [ARCHITECTURE.md]: https://github.com/Ruxex/ruxex-ratatui/blob/main/ARCHITECTURE.md
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! # Contributing
//!
//! We welcome contributions from the community! Please see our [CONTRIBUTING](../CONTRIBUTING.md)
//! guide for more details on how to get involved.
//!
//! ## License
//!
//! This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

// The layout cache only uses critical-section without std, which would otherwise trip cargo's
// unused_dependencies lint.
#[cfg(all(feature = "layout-cache", feature = "std"))]
use critical_section as _;

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod widgets;
