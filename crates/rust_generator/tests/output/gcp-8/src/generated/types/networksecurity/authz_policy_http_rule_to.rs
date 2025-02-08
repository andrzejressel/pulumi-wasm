#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthzPolicyHttpRuleTo {
    /// Describes properties of one or more targets of a request. At least one of operations or notOperations must be specified. Limited to 5 operations. A match occurs when ANY operation (in operations or notOperations) matches. Within an operation, the match follows AND semantics across fields and OR semantics within a field, i.e. a match occurs when ANY path matches AND ANY header matches and ANY method matches.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "operations")]
    pub r#operations: Box<Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleToOperation>>>,
}
