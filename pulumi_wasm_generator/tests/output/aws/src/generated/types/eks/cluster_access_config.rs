#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAccessConfig {
    /// The authentication mode for the cluster. Valid values are `CONFIG_MAP`, `API` or `API_AND_CONFIG_MAP`
    #[builder(into, default)]
    #[serde(rename = "authenticationMode")]
    pub r#authentication_mode: Box<Option<String>>,
    /// Whether or not to bootstrap the access config values to the cluster. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "bootstrapClusterCreatorAdminPermissions")]
    pub r#bootstrap_cluster_creator_admin_permissions: Box<Option<bool>>,
}