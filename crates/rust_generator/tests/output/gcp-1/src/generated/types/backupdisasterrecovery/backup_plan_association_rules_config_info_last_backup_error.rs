#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPlanAssociationRulesConfigInfoLastBackupError {
    /// (Output)
    /// The status code, which should be an enum value of [google.rpc.Code]
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<f64>>,
    /// (Output)
    /// A developer-facing error message, which should be in English.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
