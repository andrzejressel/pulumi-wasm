#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2VmSchedulingConfig {
    /// Defines whether the node is preemptible.
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// Whether the node is created under a reservation.
    #[builder(into, default)]
    #[serde(rename = "reserved")]
    pub r#reserved: Box<Option<bool>>,
}
