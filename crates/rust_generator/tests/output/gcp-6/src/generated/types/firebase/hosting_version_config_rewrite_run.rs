#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HostingVersionConfigRewriteRun {
    /// Optional. User-provided region where the Cloud Run service is hosted. Defaults to `us-central1` if not supplied.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// User-defined ID of the Cloud Run service.
    #[builder(into)]
    #[serde(rename = "serviceId")]
    pub r#service_id: Box<String>,
}
