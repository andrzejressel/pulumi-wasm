#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReportPlanReportDeliveryChannel {
    /// A list of the format of your reports: CSV, JSON, or both. If not specified, the default format is CSV.
    #[builder(into, default)]
    #[serde(rename = "formats")]
    pub r#formats: Box<Option<Vec<String>>>,
    /// The unique name of the S3 bucket that receives your reports.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// The prefix for where Backup Audit Manager delivers your reports to Amazon S3. The prefix is this part of the following path: s3://your-bucket-name/prefix/Backup/us-west-2/year/month/day/report-name. If not specified, there is no prefix.
    #[builder(into, default)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Box<Option<String>>,
}
