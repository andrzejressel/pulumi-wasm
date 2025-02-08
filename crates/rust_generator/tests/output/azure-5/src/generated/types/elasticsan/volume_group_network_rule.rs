#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeGroupNetworkRule {
    /// The action to take when the Subnet attempts to access this Elastic SAN Volume Group. The only possible value is `Allow`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The ID of the Subnet which should be allowed to access this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
