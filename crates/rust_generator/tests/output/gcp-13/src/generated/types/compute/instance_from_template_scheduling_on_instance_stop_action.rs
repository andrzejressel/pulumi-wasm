#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFromTemplateSchedulingOnInstanceStopAction {
    /// If true, the contents of any attached Local SSD disks will be discarded.
    #[builder(into, default)]
    #[serde(rename = "discardLocalSsd")]
    pub r#discard_local_ssd: Box<Option<bool>>,
}
