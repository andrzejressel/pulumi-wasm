#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AwsNodePoolUpdateSettingsSurgeSettings {
    /// Optional. The maximum number of nodes that can be created beyond the current size of the node pool during the update process.
    #[builder(into, default)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<Option<i32>>,
    /// Optional. The maximum number of nodes that can be simultaneously unavailable during the update process. A node is considered unavailable if its status is not Ready.
    #[builder(into, default)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<Option<i32>>,
}
