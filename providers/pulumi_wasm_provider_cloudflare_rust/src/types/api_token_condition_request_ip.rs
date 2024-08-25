#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ApiTokenConditionRequestIp {
    /// List of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses.
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// List of IP addresses or CIDR notation where the token should not be used from.
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
