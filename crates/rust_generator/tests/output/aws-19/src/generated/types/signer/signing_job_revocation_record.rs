#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SigningJobRevocationRecord {
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "revokedBy")]
    pub r#revoked_by: Box<Option<String>>,
}
