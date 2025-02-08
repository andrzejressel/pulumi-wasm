#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentSetParameterAction {
    /// Display name of the parameter.
    #[builder(into, default)]
    #[serde(rename = "parameter")]
    pub r#parameter: Box<Option<String>>,
    /// The new JSON-encoded value of the parameter. A null value clears the parameter.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
