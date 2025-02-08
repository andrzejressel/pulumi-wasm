#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedHardwareSecurityModuleNetworkAcls {
    /// Specifies which traffic can bypass the network rules. Possible values are `AzureServices` and `None`.
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: Box<String>,
    /// The Default Action to use. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
}
