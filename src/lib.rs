#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Core license types.
pub mod types;
/// Re-exported core license types from the [`types`] module.
pub use types::*;

/// SPDX API client.
pub mod spdx;
/// Re-exported SPDX API client from the [`spdx`] module.
pub use spdx::*;
