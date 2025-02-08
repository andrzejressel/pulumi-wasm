#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodeConfigSandboxConfig {
    /// Type of the sandbox to use for the node (e.g. 'gvisor')
    #[builder(into)]
    #[serde(rename = "sandboxType")]
    pub r#sandbox_type: Box<String>,
}
