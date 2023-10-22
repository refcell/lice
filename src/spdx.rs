use anyhow::Result;

use serde::{Deserialize, Serialize};

/// SPDX API client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SPDX;

impl SPDX {
    /// The base SPDX License API URL.
    pub const BASE_URL: &'static str = "https://spdx.org/licenses";

    /// Get a list of all SPDX licenses.
    pub async fn licenses() -> Result<crate::types::Licenses> {
        let url = format!("{}/licenses.json", Self::BASE_URL);
        reqwest::get(url)
            .await?
            .json::<crate::types::Licenses>()
            .await
            .map_err(|e| e.into())
    }

    /// Fetch a license by ID, e.g. `MIT`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use lice::spdx;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///    let license = spdx::SPDX::license("MIT").await?;
    ///    println!("{:#?}", license);
    ///    Ok(())
    /// }
    /// ```
    pub async fn license(id: &str) -> Result<crate::types::License> {
        let url = format!("{}/{}.json", Self::BASE_URL, id);
        reqwest::get(url)
            .await?
            .json::<crate::types::License>()
            .await
            .map_err(|e| e.into())
    }

    /// Fuzzy search for a license by ID, e.g. `MIT`.
    ///
    /// # Example
    /// ```rust
    /// use anyhow::Result;
    /// use lice::spdx;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let license = spdx::SPDX::fuzzy_find("mit").await?;
    ///   let license = license.ok_or(anyhow::anyhow!("no license!"))?;
    ///   assert_eq!(license.license_id, "MIT");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fuzzy_find(id: &str) -> Result<Option<crate::types::License>> {
        let licenses = Self::licenses().await?;
        Ok(licenses.match_license(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_licenses() -> Result<()> {
        let licenses = SPDX::licenses().await?;
        println!("{:#?}", licenses);
        assert!(licenses.licenses.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_single_license() -> Result<()> {
        let license = SPDX::license("MIT").await?;
        println!("{:#?}", license);
        assert_eq!(license.license_id, "MIT");
        Ok(())
    }
}
