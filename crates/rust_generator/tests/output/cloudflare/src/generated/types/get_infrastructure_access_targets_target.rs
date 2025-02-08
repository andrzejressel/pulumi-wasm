#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInfrastructureAccessTargetsTarget {
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
    pub r#ip: Box<super::types::GetInfrastructureAccessTargetsTargetIp>,
    /// The date and time at which the target was last modified.
    #[builder(into)]
    #[serde(rename = "modifiedAt")]
    pub r#modified_at: Box<String>,
}
