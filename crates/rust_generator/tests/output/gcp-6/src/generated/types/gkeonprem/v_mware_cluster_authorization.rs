#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterAuthorization {
    /// Users that will be granted the cluster-admin role on the cluster, providing
    /// full access to the cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "adminUsers")]
    pub r#admin_users: Box<Option<Vec<super::super::types::gkeonprem::VMwareClusterAuthorizationAdminUser>>>,
}
