#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodePoolNodeConfigSecondaryBootDisk {
    /// Disk image to create the secondary boot disk from
    #[builder(into)]
    #[serde(rename = "diskImage")]
    pub r#disk_image: Box<String>,
    /// Mode for how the secondary boot disk is used.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
