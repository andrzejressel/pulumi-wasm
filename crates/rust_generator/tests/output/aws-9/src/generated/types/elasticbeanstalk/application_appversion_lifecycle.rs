#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationAppversionLifecycle {
    /// Set to `true` to delete a version's source bundle from S3 when the application version is deleted.
    #[builder(into, default)]
    #[serde(rename = "deleteSourceFromS3")]
    pub r#delete_source_from_s_3: Box<Option<bool>>,
    /// The number of days to retain an application version ('max_age_in_days' and 'max_count' cannot be enabled simultaneously.).
    #[builder(into, default)]
    #[serde(rename = "maxAgeInDays")]
    pub r#max_age_in_days: Box<Option<i32>>,
    /// The maximum number of application versions to retain ('max_age_in_days' and 'max_count' cannot be enabled simultaneously.).
    #[builder(into, default)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<Option<i32>>,
    /// The ARN of an IAM service role under which the application version is deleted.  Elastic Beanstalk must have permission to assume this role.
    #[builder(into)]
    #[serde(rename = "serviceRole")]
    pub r#service_role: Box<String>,
}
