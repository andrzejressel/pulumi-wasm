#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetEventDestinationEventDestinationCloudWatchDestination {
    /// An array of objects that define the dimensions to use when you send email events to Amazon CloudWatch. See `dimension_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "dimensionConfigurations")]
    pub r#dimension_configurations: Box<Vec<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration>>,
}
