#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZeroTrustInfrastructureAccessTargetsTarget {
    /// The account identifier to target for the resource.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// The date and time at which the target was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Box<String>,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// The identifier of this resource. This is target's unique identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<crate::types::GetZeroTrustInfrastructureAccessTargetsTargetIp>,
    /// The date and time at which the target was last modified.
    #[builder(into)]
    #[serde(rename = "modifiedAt")]
    pub r#modified_at: Box<String>,
}
