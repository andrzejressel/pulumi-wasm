#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleMatchHttpMatch {
    /// The header matches. Matches incoming requests with rule based on request header value before applying rule action.
    #[builder(into, default)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Box<Option<Vec<super::super::types::vpclattice::ListenerRuleMatchHttpMatchHeaderMatch>>>,
    /// The HTTP method type.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// The path match.
    #[builder(into, default)]
    #[serde(rename = "pathMatch")]
    pub r#path_match: Box<Option<super::super::types::vpclattice::ListenerRuleMatchHttpMatchPathMatch>>,
}
