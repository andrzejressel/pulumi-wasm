#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationInputsProcessingConfiguration {
    /// The Lambda function configuration. See Lambda below for more details.
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfigurationLambda>,
}
