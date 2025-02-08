#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AzureClusterAzureServicesAuthentication {
    /// The Azure Active Directory Application ID for Authentication configuration.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Box<String>,
    /// The Azure Active Directory Tenant ID for Authentication configuration.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
