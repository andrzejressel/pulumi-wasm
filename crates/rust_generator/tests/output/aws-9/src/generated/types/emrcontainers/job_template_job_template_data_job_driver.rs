#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateDataJobDriver {
    /// The job driver for job type.
    #[builder(into, default)]
    #[serde(rename = "sparkSqlJobDriver")]
    pub r#spark_sql_job_driver: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriverSparkSqlJobDriver>>,
    /// The job driver parameters specified for spark submit.
    #[builder(into, default)]
    #[serde(rename = "sparkSubmitJobDriver")]
    pub r#spark_submit_job_driver: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver>>,
}
