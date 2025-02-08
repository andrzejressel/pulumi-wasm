#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2QueuedResourceTpu {
    /// The TPU node(s) being requested.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeSpecs")]
    pub r#node_specs: Box<Option<Vec<super::super::types::tpu::V2QueuedResourceTpuNodeSpec>>>,
}
