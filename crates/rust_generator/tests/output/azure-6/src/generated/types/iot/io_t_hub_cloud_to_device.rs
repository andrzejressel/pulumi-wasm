#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubCloudToDevice {
    /// The default time to live for cloud-to-device messages, specified as an [ISO 8601 timespan duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This value must be between 1 minute and 48 hours. Defaults to `PT1H`.
    #[builder(into, default)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<Option<String>>,
    /// A `feedback` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "feedbacks")]
    pub r#feedbacks: Box<Option<Vec<super::super::types::iot::IoTHubCloudToDeviceFeedback>>>,
    /// The maximum delivery count for cloud-to-device per-device queues. This value must be between `1` and `100`. Defaults to `10`.
    #[builder(into, default)]
    #[serde(rename = "maxDeliveryCount")]
    pub r#max_delivery_count: Box<Option<i32>>,
}
