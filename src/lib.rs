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

/// Get fetches a license by ID, e.g. `MIT`.
/// Internally, this is a wrapper around [`SPDX::license`],
/// performing a fuzzy search for the given ID, and merging
/// the results with the full license details.
pub async fn get(id: &str) -> anyhow::Result<License> {
    use merge::Merge;
    let license = SPDX::fuzzy_find(id).await?;
    let mut license = license.ok_or(anyhow::anyhow!("no license!"))?;
    let details = SPDX::license(&license.license_id).await?;
    license.merge(details);
    Ok(license)
}
