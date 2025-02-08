#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CrawlerS3Target {
    /// The name of a connection which allows crawler to access data in S3 within a VPC.
    #[builder(into, default)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<Option<String>>,
    /// The ARN of the dead-letter SQS queue.
    #[builder(into, default)]
    #[serde(rename = "dlqEventQueueArn")]
    pub r#dlq_event_queue_arn: Box<Option<String>>,
    /// The ARN of the SQS queue to receive S3 notifications from.
    #[builder(into, default)]
    #[serde(rename = "eventQueueArn")]
    pub r#event_queue_arn: Box<Option<String>>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<String>>>,
    /// The path to the Amazon S3 target.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Sets the number of files in each leaf folder to be crawled when crawling sample files in a dataset. If not set, all the files are crawled. A valid value is an integer between 1 and 249.
    #[builder(into, default)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: Box<Option<i32>>,
}
