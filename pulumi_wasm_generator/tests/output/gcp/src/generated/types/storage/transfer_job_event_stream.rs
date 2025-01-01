#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobEventStream {
    /// Specifies the data and time at which Storage Transfer Service stops listening for events from this stream. After this time, any transfers in progress will complete, but no new transfers are initiated.A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "eventStreamExpirationTime")]
    pub r#event_stream_expiration_time: Box<Option<String>>,
    /// Specifies the date and time that Storage Transfer Service starts listening for events from this stream. If no start time is specified or start time is in the past, Storage Transfer Service starts listening immediately. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "eventStreamStartTime")]
    pub r#event_stream_start_time: Box<Option<String>>,
    /// Specifies a unique name of the resource such as AWS SQS ARN in the form 'arn:aws:sqs:region:account_id:queue_name', or Pub/Sub subscription resource name in the form 'projects/{project}/subscriptions/{sub}'.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
