#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppImageConfigKernelGatewayImageConfigKernelSpec {
    /// The display name of the kernel.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// The name of the kernel.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
