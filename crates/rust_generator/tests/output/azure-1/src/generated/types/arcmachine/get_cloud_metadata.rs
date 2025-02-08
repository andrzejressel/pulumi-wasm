#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCloudMetadata {
    /// Specifies the cloud provider. For example `Azure`, `AWS` and `GCP`.
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: Box<String>,
}
