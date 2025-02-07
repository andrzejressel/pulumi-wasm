#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterServicePrincipal {
    /// The Client ID of the user-defined Managed Identity assigned to the Kubelets.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
}
