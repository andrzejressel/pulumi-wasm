#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustAccessPolicyConnectionRulesSsh {
    /// Contains the Unix usernames that may be used when connecting over SSH.
    #[builder(into)]
    #[serde(rename = "usernames")]
    pub r#usernames: Box<Vec<String>>,
}
