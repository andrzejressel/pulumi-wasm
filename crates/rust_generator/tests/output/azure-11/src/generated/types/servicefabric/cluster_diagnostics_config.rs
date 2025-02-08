#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterDiagnosticsConfig {
    /// The Blob Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "blobEndpoint")]
    pub r#blob_endpoint: Box<String>,
    /// The protected diagnostics storage key name, such as `StorageAccountKey1`.
    #[builder(into)]
    #[serde(rename = "protectedAccountKeyName")]
    pub r#protected_account_key_name: Box<String>,
    /// The Queue Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "queueEndpoint")]
    pub r#queue_endpoint: Box<String>,
    /// The name of the Storage Account where the Diagnostics should be sent to.
    #[builder(into)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Box<String>,
    /// The Table Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "tableEndpoint")]
    pub r#table_endpoint: Box<String>,
}
