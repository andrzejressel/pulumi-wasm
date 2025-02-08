#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecurityDeviceGroupAllowRule {
    /// Specifies which IP is not allowed to be connected to in current device group for inbound connection.
    #[builder(into, default)]
    #[serde(rename = "connectionFromIpsNotAlloweds")]
    pub r#connection_from_ips_not_alloweds: Box<Option<Vec<String>>>,
    /// Specifies which IP is not allowed to be connected to in current device group for outbound connection.
    #[builder(into, default)]
    #[serde(rename = "connectionToIpsNotAlloweds")]
    pub r#connection_to_ips_not_alloweds: Box<Option<Vec<String>>>,
    /// Specifies which local user is not allowed to login in current device group.
    #[builder(into, default)]
    #[serde(rename = "localUsersNotAlloweds")]
    pub r#local_users_not_alloweds: Box<Option<Vec<String>>>,
    /// Specifies which process is not allowed to be executed in current device group.
    #[builder(into, default)]
    #[serde(rename = "processesNotAlloweds")]
    pub r#processes_not_alloweds: Box<Option<Vec<String>>>,
}
