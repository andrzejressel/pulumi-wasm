#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetBootDiagnostics {
    /// Whether to enable boot diagnostics for the virtual machine. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Blob endpoint for the storage account to hold the virtual machine's diagnostic files. This must be the root of a storage account, and not a storage container.
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: Box<String>,
}
