#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpokeLinkedVpnTunnels {
    /// IP ranges allowed to be included during import from hub (does not control transit connectivity).
    /// The only allowed value for now is "ALL_IPV4_RANGES".
    #[builder(into, default)]
    #[serde(rename = "includeImportRanges")]
    pub r#include_import_ranges: Box<Option<Vec<String>>>,
    /// A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations.
    #[builder(into)]
    #[serde(rename = "siteToSiteDataTransfer")]
    pub r#site_to_site_data_transfer: Box<bool>,
    /// The URIs of linked VPN tunnel resources.
    #[builder(into)]
    #[serde(rename = "uris")]
    pub r#uris: Box<Vec<String>>,
}
