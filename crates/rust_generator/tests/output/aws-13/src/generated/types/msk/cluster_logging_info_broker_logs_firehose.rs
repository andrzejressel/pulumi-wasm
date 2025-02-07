#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterLoggingInfoBrokerLogsFirehose {
    /// Name of the Kinesis Data Firehose delivery stream to deliver logs to.
    #[builder(into, default)]
    #[serde(rename = "deliveryStream")]
    pub r#delivery_stream: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
