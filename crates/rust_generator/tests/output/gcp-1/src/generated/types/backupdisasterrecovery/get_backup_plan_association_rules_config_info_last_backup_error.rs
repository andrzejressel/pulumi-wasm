#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackupPlanAssociationRulesConfigInfoLastBackupError {
    /// The status code, which should be an enum value of [google.rpc.Code]
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<f64>,
    /// A developer-facing error message, which should be in English.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<String>,
}
