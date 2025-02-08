#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipeSourceParametersKinesisStreamParameters {
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    /// Define the target queue to send dead-letter queue events to. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "deadLetterConfig")]
    pub r#dead_letter_config: Box<Option<super::super::types::pipes::PipeSourceParametersKinesisStreamParametersDeadLetterConfig>>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into, default)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Box<Option<i32>>,
    /// Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records. Maximum value of 604,800.
    #[builder(into, default)]
    #[serde(rename = "maximumRecordAgeInSeconds")]
    pub r#maximum_record_age_in_seconds: Box<Option<i32>>,
    /// Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source. Maximum value of 10,000.
    #[builder(into, default)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Box<Option<i32>>,
    /// Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch. Valid values: AUTOMATIC_BISECT.
    #[builder(into, default)]
    #[serde(rename = "onPartialBatchItemFailure")]
    pub r#on_partial_batch_item_failure: Box<Option<String>>,
    /// The number of batches to process concurrently from each shard. The default value is 1. Maximum value of 10.
    #[builder(into, default)]
    #[serde(rename = "parallelizationFactor")]
    pub r#parallelization_factor: Box<Option<i32>>,
    /// The position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
    #[builder(into)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Box<String>,
    /// With StartingPosition set to AT_TIMESTAMP, the time from which to start reading, in Unix time seconds.
    #[builder(into, default)]
    #[serde(rename = "startingPositionTimestamp")]
    pub r#starting_position_timestamp: Box<Option<String>>,
}
