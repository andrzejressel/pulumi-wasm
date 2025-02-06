#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherRouteRuleMatchRuleHeaderMatchRangeMatch {
    /// The end of the range (exclusive).
    #[builder(into)]
    #[serde(rename = "rangeEnd")]
    pub r#range_end: Box<i32>,
    /// The start of the range (inclusive).
    #[builder(into)]
    #[serde(rename = "rangeStart")]
    pub r#range_start: Box<i32>,
}
