#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeTemplateAccelerator {
    /// The number of the guest accelerator cards exposed to this
    /// node template.
    #[builder(into, default)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<Option<i32>>,
    /// Full or partial URL of the accelerator type resource to expose
    /// to this node template.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
}
