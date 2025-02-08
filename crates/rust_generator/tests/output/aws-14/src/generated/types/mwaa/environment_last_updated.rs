#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentLastUpdated {
    /// The Created At date of the MWAA Environment
    #[builder(into, default)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "errors")]
    pub r#errors: Box<Option<Vec<super::super::types::mwaa::EnvironmentLastUpdatedError>>>,
    /// The status of the Amazon MWAA Environment
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
