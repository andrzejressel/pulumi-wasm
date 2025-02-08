#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerPredicate {
    /// A list of the conditions that determine when the trigger will fire. See Conditions.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::glue::TriggerPredicateCondition>>,
    /// How to handle multiple conditions. Defaults to `AND`. Valid values are `AND` or `ANY`.
    #[builder(into, default)]
    #[serde(rename = "logical")]
    pub r#logical: Box<Option<String>>,
}
