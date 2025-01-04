#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalysisParameters {
    /// A list of parameters that have a data type of date-time. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DateTimeParameter.html).
    #[builder(into, default)]
    #[serde(rename = "dateTimeParameters")]
    pub r#date_time_parameters: Box<Option<Vec<super::super::types::quicksight::AnalysisParametersDateTimeParameter>>>,
    /// A list of parameters that have a data type of decimal. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DecimalParameter.html).
    #[builder(into, default)]
    #[serde(rename = "decimalParameters")]
    pub r#decimal_parameters: Box<Option<Vec<super::super::types::quicksight::AnalysisParametersDecimalParameter>>>,
    /// A list of parameters that have a data type of integer. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_IntegerParameter.html).
    #[builder(into, default)]
    #[serde(rename = "integerParameters")]
    pub r#integer_parameters: Box<Option<Vec<super::super::types::quicksight::AnalysisParametersIntegerParameter>>>,
    /// A list of parameters that have a data type of string. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_StringParameter.html).
    #[builder(into, default)]
    #[serde(rename = "stringParameters")]
    pub r#string_parameters: Box<Option<Vec<super::super::types::quicksight::AnalysisParametersStringParameter>>>,
}
