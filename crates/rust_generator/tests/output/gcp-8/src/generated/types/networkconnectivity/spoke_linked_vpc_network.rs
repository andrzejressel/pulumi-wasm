#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpokeLinkedVpcNetwork {
    /// IP ranges encompassing the subnets to be excluded from peering.
    #[builder(into, default)]
    #[serde(rename = "excludeExportRanges")]
    pub r#exclude_export_ranges: Box<Option<Vec<String>>>,
    /// IP ranges allowed to be included from peering.
    #[builder(into, default)]
    #[serde(rename = "includeExportRanges")]
    pub r#include_export_ranges: Box<Option<Vec<String>>>,
    /// The URI of the VPC network resource.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
