#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration {
    /// Describes the metadata sent to the HTTP endpoint destination. See `common_attributes` block below for details.
    #[builder(into, default)]
    #[serde(rename = "commonAttributes")]
    pub r#common_attributes: Box<Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfigurationCommonAttribute>>>,
    /// Kinesis Data Firehose uses the content encoding to compress the body of a request before sending the request to the destination. Valid values are `NONE` and `GZIP`.  Default value is `NONE`.
    #[builder(into, default)]
    #[serde(rename = "contentEncoding")]
    pub r#content_encoding: Box<Option<String>>,
}
