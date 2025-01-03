#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerMatchingCriteria {
    /// Required. The name of a CloudEvents attribute. Currently, only a subset of attributes are supported for filtering. All triggers MUST provide a filter for the 'type' attribute.
    #[builder(into)]
    #[serde(rename = "attribute")]
    pub r#attribute: Box<String>,
    /// Optional. The operator used for matching the events with the value of the filter. If not specified, only events that have an exact key-value pair specified in the filter are matched. The only allowed value is `match-path-pattern`.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// Required. The value for the attribute. See https://cloud.google.com/eventarc/docs/creating-triggers#trigger-gcloud for available values.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
