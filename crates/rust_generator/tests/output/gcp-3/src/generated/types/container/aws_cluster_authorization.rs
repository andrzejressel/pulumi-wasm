#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterAuthorization {
    /// Groups of users that can perform operations as a cluster admin. A managed ClusterRoleBinding will be created to grant the `cluster-admin` ClusterRole to the groups. Up to ten admin groups can be provided. For more info on RBAC, see https://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles
    #[builder(into, default)]
    #[serde(rename = "adminGroups")]
    pub r#admin_groups: Box<Option<Vec<super::super::types::container::AwsClusterAuthorizationAdminGroup>>>,
    /// Users to perform operations as a cluster admin. A managed ClusterRoleBinding will be created to grant the `cluster-admin` ClusterRole to the users. Up to ten admin users can be provided. For more info on RBAC, see https://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles
    #[builder(into)]
    #[serde(rename = "adminUsers")]
    pub r#admin_users: Box<Vec<super::super::types::container::AwsClusterAuthorizationAdminUser>>,
}
