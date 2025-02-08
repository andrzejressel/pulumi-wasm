#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterLogging {
    /// The name of an existing S3 bucket where the log files are to be stored. Must be in the same region as the cluster and the cluster must have read bucket and put object permissions.
    /// For more information on the permissions required for the bucket, please read the AWS [documentation](http://docs.aws.amazon.com/redshift/latest/mgmt/db-auditing.html#db-auditing-enable-logging)
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// Enables logging information such as queries and connection attempts, for the specified Amazon Redshift cluster.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
    /// The log destination type. An enum with possible values of `s3` and `cloudwatch`.
    #[builder(into, default)]
    #[serde(rename = "logDestinationType")]
    pub r#log_destination_type: Box<Option<String>>,
    /// The collection of exported log types. Log types include the connection log, user log and user activity log. Required when `log_destination_type` is `cloudwatch`. Valid log types are `connectionlog`, `userlog`, and `useractivitylog`.
    #[builder(into, default)]
    #[serde(rename = "logExports")]
    pub r#log_exports: Box<Option<Vec<String>>>,
    /// The prefix applied to the log file names.
    #[builder(into, default)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Box<Option<String>>,
}
