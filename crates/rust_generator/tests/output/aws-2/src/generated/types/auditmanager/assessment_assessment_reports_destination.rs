#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssessmentAssessmentReportsDestination {
    /// Destination of the assessment report. This value be in the form `s3://{bucket_name}`.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// Destination type. Currently, `S3` is the only valid value.
    #[builder(into)]
    #[serde(rename = "destinationType")]
    pub r#destination_type: Box<String>,
}
