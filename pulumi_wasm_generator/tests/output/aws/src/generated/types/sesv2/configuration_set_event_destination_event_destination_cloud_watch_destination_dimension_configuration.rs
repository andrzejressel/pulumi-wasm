#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
    /// The default value of the dimension that is published to Amazon CloudWatch if you don't provide the value of the dimension when you send an email.
    #[builder(into)]
    #[serde(rename = "defaultDimensionValue")]
    pub r#default_dimension_value: Box<String>,
    /// The name of an Amazon CloudWatch dimension associated with an email sending metric.
    #[builder(into)]
    #[serde(rename = "dimensionName")]
    pub r#dimension_name: Box<String>,
    /// The location where the Amazon SES API v2 finds the value of a dimension to publish to Amazon CloudWatch. Valid values: `MESSAGE_TAG`, `EMAIL_HEADER`, `LINK_TAG`.
    #[builder(into)]
    #[serde(rename = "dimensionValueSource")]
    pub r#dimension_value_source: Box<String>,
}
