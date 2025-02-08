#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmInstantRestoreResourceGroup {
    /// The prefix for the `instant_restore_resource_group` name.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    /// The suffix for the `instant_restore_resource_group` name.
    #[builder(into, default)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
