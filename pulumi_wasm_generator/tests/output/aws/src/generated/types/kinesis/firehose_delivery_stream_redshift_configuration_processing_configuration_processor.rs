#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamRedshiftConfigurationProcessingConfigurationProcessor {
    /// Specifies the processor parameters as multiple blocks. See `parameters` block below for details.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationProcessingConfigurationProcessorParameter>>>,
    /// The type of processor. Valid Values: `RecordDeAggregation`, `Lambda`, `MetadataExtraction`, `AppendDelimiterToRecord`, `Decompression`, `CloudWatchLogProcessing`. Validation is done against [AWS SDK constants](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/firehose/types#ProcessorType); so values not explicitly listed may also work.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}