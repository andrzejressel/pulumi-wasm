#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxFlowTransitionRouteTriggerFulfillmentConditionalCase {
    /// A JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.
    /// See [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema.
    #[builder(into, default)]
    #[serde(rename = "cases")]
    pub r#cases: Box<Option<String>>,
}
