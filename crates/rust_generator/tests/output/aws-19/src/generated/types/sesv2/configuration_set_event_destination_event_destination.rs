#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationSetEventDestinationEventDestination {
    /// An object that defines an Amazon CloudWatch destination for email events. See `cloud_watch_destination` Block for details.
    #[builder(into, default)]
    #[serde(rename = "cloudWatchDestination")]
    pub r#cloud_watch_destination: Box<Option<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationCloudWatchDestination>>,
    /// When the event destination is enabled, the specified event types are sent to the destinations. Default: `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "eventBridgeDestination")]
    pub r#event_bridge_destination: Box<Option<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationEventBridgeDestination>>,
    /// An object that defines an Amazon Kinesis Data Firehose destination for email events. See `kinesis_firehose_destination` Block for details.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehoseDestination")]
    pub r#kinesis_firehose_destination: Box<Option<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination>>,
    /// An array that specifies which events the Amazon SES API v2 should send to the destinations. Valid values: `SEND`, `REJECT`, `BOUNCE`, `COMPLAINT`, `DELIVERY`, `OPEN`, `CLICK`, `RENDERING_FAILURE`, `DELIVERY_DELAY`, `SUBSCRIPTION`.
    #[builder(into)]
    #[serde(rename = "matchingEventTypes")]
    pub r#matching_event_types: Box<Vec<String>>,
    /// An object that defines an Amazon Pinpoint project destination for email events. See `pinpoint_destination` Block for details.
    #[builder(into, default)]
    #[serde(rename = "pinpointDestination")]
    pub r#pinpoint_destination: Box<Option<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationPinpointDestination>>,
    /// An object that defines an Amazon SNS destination for email events. See `sns_destination` Block for details.
    #[builder(into, default)]
    #[serde(rename = "snsDestination")]
    pub r#sns_destination: Box<Option<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationSnsDestination>>,
}
