#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedClusterAuthenticationActiveDirectory {
    /// The ID of the Client Application.
    #[builder(into)]
    #[serde(rename = "clientApplicationId")]
    pub r#client_application_id: Box<String>,
    /// The ID of the Cluster Application.
    #[builder(into)]
    #[serde(rename = "clusterApplicationId")]
    pub r#cluster_application_id: Box<String>,
    /// The ID of the Tenant.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
