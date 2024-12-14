#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiTokenConditionRequestIp {
    /// List of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// List of IP addresses or CIDR notation where the token should not be used from.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
