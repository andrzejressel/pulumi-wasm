#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthzPolicyHttpRuleFromNotSource {
    /// A list of identities derived from the client's certificate. This field will not match on a request unless mutual TLS is enabled for the Forwarding rule or Gateway. Each identity is a string whose value is matched against the URI SAN, or DNS SAN or the subject field in the client's certificate. The match can be exact, prefix, suffix or a substring match. One of exact, prefix, suffix or contains must be specified.
    /// Limited to 5 principals.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "principals")]
    pub r#principals: Box<Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromNotSourcePrincipal>>>,
    /// A list of resources to match against the resource of the source VM of a request.
    /// Limited to 5 resources.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromNotSourceResource>>>,
}
