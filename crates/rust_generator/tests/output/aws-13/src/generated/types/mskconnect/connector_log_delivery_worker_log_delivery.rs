#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorLogDeliveryWorkerLogDelivery {
    /// Details about delivering logs to Amazon CloudWatch Logs. See `cloudwatch_logs` Block for details.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs>>,
    /// Details about delivering logs to Amazon Kinesis Data Firehose. See `firehose` Block for details.
    #[builder(into, default)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryFirehose>>,
    /// Details about delivering logs to Amazon S3. See `s3` Block for deetails.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryS3>>,
}
