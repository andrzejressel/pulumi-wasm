#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyCustomRuleMatchConditionMatchVariable {
    /// Describes field of the matchVariable collection
    #[builder(into, default)]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<String>>,
    /// The name of the Match Variable. Possible values are `RemoteAddr`, `RequestMethod`, `QueryString`, `PostArgs`, `RequestUri`, `RequestHeaders`, `RequestBody` and `RequestCookies`.
    #[builder(into)]
    #[serde(rename = "variableName")]
    pub r#variable_name: Box<String>,
}
