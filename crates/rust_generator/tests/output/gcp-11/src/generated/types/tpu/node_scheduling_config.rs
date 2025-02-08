#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NodeSchedulingConfig {
    /// Defines whether the TPU instance is preemptible.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<bool>,
}
