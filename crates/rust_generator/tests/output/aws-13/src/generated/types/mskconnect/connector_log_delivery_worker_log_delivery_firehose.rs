#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorLogDeliveryWorkerLogDeliveryFirehose {
    /// The name of the Kinesis Data Firehose delivery stream that is the destination for log delivery.
    #[builder(into, default)]
    #[serde(rename = "deliveryStream")]
    pub r#delivery_stream: Box<Option<String>>,
    /// Specifies whether connector logs get delivered to Amazon Kinesis Data Firehose.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
