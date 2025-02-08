#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrailAdvancedEventSelector {
    /// Specifies the selector statements in an advanced event selector. Fields documented below.
    #[builder(into)]
    #[serde(rename = "fieldSelectors")]
    pub r#field_selectors: Box<Vec<super::super::types::cloudtrail::TrailAdvancedEventSelectorFieldSelector>>,
    /// Name of the trail.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
