#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateNodeSelector {
    /// The GPU to attach to an instance. See https://cloud.google.com/run/docs/configuring/services/gpu for configuring GPU.
    #[builder(into)]
    #[serde(rename = "accelerator")]
    pub r#accelerator: Box<String>,
}
