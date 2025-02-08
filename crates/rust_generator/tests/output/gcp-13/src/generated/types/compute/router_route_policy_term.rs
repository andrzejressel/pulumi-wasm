#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RouterRoutePolicyTerm {
    /// 'CEL expressions to evaluate to modify a route when this term matches.'\
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<super::super::types::compute::RouterRoutePolicyTermAction>>>,
    /// CEL expression evaluated against a route to determine if this term applies (see Policy Language). When not set, the term applies to all routes.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::compute::RouterRoutePolicyTermMatch>>,
    /// The evaluation priority for this term, which must be between 0 (inclusive) and 231 (exclusive), and unique within the list.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
