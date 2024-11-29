#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessPolicyConnectionRulesSsh {
    /// Contains the Unix usernames that may be used when connecting over SSH.
    #[builder(into)]
    #[serde(rename = "usernames")]
    pub r#usernames: Box<Vec<String>>,
}
