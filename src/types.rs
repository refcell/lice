use serde::Deserialize;

/// Licenses Response Type.
#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Licenses {
    /// License list version.
    pub license_list_version: String,
    /// List of licenses.
    pub licenses: Vec<License>,
    /// The release date.
    pub release_date: String,
}

/// The License Object.
///
/// The License Object is a JSON object that contains the details of a license.
///
/// When the license is returned as part of a list of licenses,
/// the following fields are present:
/// - `reference`
/// - `detailsUrl`
/// - `referenceNumber`
///
/// When the license is returned as a single license,
/// the following fields are present:
/// - `licenseText`
/// - `standardLicenseTemplate`
/// - `licenseTextHtml`
/// - `crossRef`
#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The license name.
    pub name: String,
    /// The license ID.
    pub license_id: String,
    /// A list of URLs to other resources related to the license.
    pub see_also: Vec<String>,
    /// Whether or not the license is OSI approved.
    pub is_osi_approved: bool,
    /// Whether or not the license is FSF libre.
    pub is_fsf_libre: Option<bool>,
    /// Whether or not the license is deprecated.
    pub is_deprecated_license_id: bool,

    /// A reference to the license.
    pub reference: Option<String>,
    /// The URL to the license details.
    pub details_url: Option<String>,
    /// The license reference number.
    pub reference_number: Option<u32>,

    /// License text.
    pub license_text: Option<String>,
    /// Standard license template.
    pub standard_license_template: Option<String>,
    /// License text in HTML format.
    pub license_text_html: Option<String>,
    /// A list of cross references.
    pub cross_ref: Option<Vec<CrossRef>>,
}

/// Cross Reference.
#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CrossRef {
    /// The match.
    pub r#match: String,
    /// The URL.
    pub url: String,
    /// Whether or not the URL is valid.
    pub is_valid: bool,
    /// Whether or not the URL is live.
    pub is_live: bool,
    /// The timestamp.
    pub timestamp: String,
    /// Whether or not the URL is a wayback link.
    pub is_way_back_link: bool,
    /// The order.
    pub order: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mit_license_serde() {
        let data = r#"
        {
              "isDeprecatedLicenseId": false,
              "isFsfLibre": true,
              "licenseText": "...",
              "standardLicenseTemplate": "...",
              "name": "MIT License",
              "licenseId": "MIT",
              "crossRef": [
                {
                  "match": "N/A",
                  "url": "https://opensource.org/licenses/MIT",
                  "isValid": true,
                  "isLive": false,
                  "timestamp": "2023-10-05T15:11:54Z",
                  "isWayBackLink": false,
                  "order": 0
                }
              ],
              "seeAlso": [ "https://opensource.org/licenses/MIT" ],
              "isOsiApproved": true,
              "licenseTextHtml": "..."
        }"#;
        let deserialized: License = serde_json::from_str(data).unwrap();
        let expected = License {
            name: String::from("MIT License"),
            license_id: String::from("MIT"),
            see_also: vec![String::from("https://opensource.org/licenses/MIT")],
            is_osi_approved: true,
            is_fsf_libre: Some(true),
            is_deprecated_license_id: false,
            license_text: Some(String::from("...")),
            standard_license_template: Some(String::from("...")),
            license_text_html: Some(String::from("...")),
            cross_ref: Some(vec![CrossRef {
                r#match: String::from("N/A"),
                url: String::from("https://opensource.org/licenses/MIT"),
                is_valid: true,
                is_live: false,
                timestamp: String::from("2023-10-05T15:11:54Z"),
                is_way_back_link: false,
                order: 0,
            }]),
            reference: None,
            details_url: None,
            reference_number: None,
        };
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_licenses() {
        let data = r#"
        {
            "licenseListVersion": "3.16",
            "licenses": [
                {
                  "reference": "https://spdx.org/licenses/MIT.html",
                  "isDeprecatedLicenseId": false,
                  "detailsUrl": "https://spdx.org/licenses/MIT.json",
                  "referenceNumber": 246,
                  "name": "MIT License",
                  "licenseId": "MIT",
                  "seeAlso": [
                    "https://opensource.org/licenses/MIT"
                  ],
                  "isOsiApproved": true,
                  "isFsfLibre": true
                }
            ],
            "releaseDate": "2022-02-06"
        }"#;
        let deserialized: Licenses = serde_json::from_str(data).unwrap();
        let expected_license = License {
            name: String::from("MIT License"),
            license_id: String::from("MIT"),
            see_also: vec![String::from("https://opensource.org/licenses/MIT")],
            is_osi_approved: true,
            is_fsf_libre: Some(true),
            is_deprecated_license_id: false,
            license_text: None,
            standard_license_template: None,
            license_text_html: None,
            cross_ref: None,
            reference: Some(String::from("https://spdx.org/licenses/MIT.html")),
            details_url: Some(String::from("https://spdx.org/licenses/MIT.json")),
            reference_number: Some(246),
        };
        let expected = Licenses {
            license_list_version: String::from("3.16"),
            licenses: vec![expected_license],
            release_date: String::from("2022-02-06"),
        };
        assert_eq!(deserialized, expected);
    }
}
