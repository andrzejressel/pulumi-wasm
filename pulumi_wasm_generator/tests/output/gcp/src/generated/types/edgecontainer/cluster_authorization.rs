#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAuthorization {
    /// User that will be granted the cluster-admin role on the cluster, providing
    /// full access to the cluster. Currently, this is a singular field, but will
    /// be expanded to allow multiple admins in the future.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "adminUsers")]
    pub r#admin_users: Box<super::super::types::edgecontainer::ClusterAuthorizationAdminUsers>,
}
