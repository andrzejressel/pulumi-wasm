#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobStatus {
    /// (Output)
    /// Final error result of the job. If present, indicates that the job has completed and was unsuccessful.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorResults")]
    pub r#error_results: Box<Option<Vec<super::super::types::bigquery::JobStatusErrorResult>>>,
    /// (Output)
    /// The first errors encountered during the running of the job. The final message
    /// includes the number of errors that caused the process to stop. Errors here do
    /// not necessarily mean that the job has not completed or was unsuccessful.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errors")]
    pub r#errors: Box<Option<Vec<super::super::types::bigquery::JobStatusError>>>,
    /// (Output)
    /// Running state of the job. Valid states include 'PENDING', 'RUNNING', and 'DONE'.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
