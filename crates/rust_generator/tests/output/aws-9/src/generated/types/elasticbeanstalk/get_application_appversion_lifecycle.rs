#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationAppversionLifecycle {
    /// Specifies whether delete a version's source bundle from S3 when the application version is deleted.
    #[builder(into)]
    #[serde(rename = "deleteSourceFromS3")]
    pub r#delete_source_from_s_3: Box<bool>,
    /// Number of days to retain an application version.
    #[builder(into)]
    #[serde(rename = "maxAgeInDays")]
    pub r#max_age_in_days: Box<i32>,
    /// Maximum number of application versions to retain.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<i32>,
    /// ARN of an IAM service role under which the application version is deleted.  Elastic Beanstalk must have permission to assume this role.
    #[builder(into)]
    #[serde(rename = "serviceRole")]
    pub r#service_role: Box<String>,
}
