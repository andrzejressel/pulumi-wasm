#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListenerRuleMatchHttpMatchHeaderMatchMatch {
    /// Specifies a contains type match.
    #[builder(into, default)]
    #[serde(rename = "contains")]
    pub r#contains: Box<Option<String>>,
    /// Specifies an exact type match.
    #[builder(into, default)]
    #[serde(rename = "exact")]
    pub r#exact: Box<Option<String>>,
    /// Specifies a prefix type match. Matches the value with the prefix.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
