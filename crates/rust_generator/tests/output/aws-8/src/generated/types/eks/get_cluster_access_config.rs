#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterAccessConfig {
    /// Values returned are `CONFIG_MAP`, `API` or `API_AND_CONFIG_MAP`
    #[builder(into)]
    #[serde(rename = "authenticationMode")]
    pub r#authentication_mode: Box<String>,
    /// Default to `true`.
    #[builder(into)]
    #[serde(rename = "bootstrapClusterCreatorAdminPermissions")]
    pub r#bootstrap_cluster_creator_admin_permissions: Box<bool>,
}
