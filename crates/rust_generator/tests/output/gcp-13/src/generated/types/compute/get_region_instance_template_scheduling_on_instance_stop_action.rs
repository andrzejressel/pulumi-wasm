#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRegionInstanceTemplateSchedulingOnInstanceStopAction {
    /// If true, the contents of any attached Local SSD disks will be discarded.
    #[builder(into)]
    #[serde(rename = "discardLocalSsd")]
    pub r#discard_local_ssd: Box<bool>,
}
