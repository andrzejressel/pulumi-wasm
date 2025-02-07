#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FluxConfigurationBlobStorage {
    /// Specifies the account key (shared key) to access the storage account.
    #[builder(into, default)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Box<Option<String>>,
    /// Specifies the Azure Blob container ID.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: Box<String>,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets.
    #[builder(into, default)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Box<Option<String>>,
    /// Specifies the shared access token to access the storage container.
    #[builder(into, default)]
    #[serde(rename = "sasToken")]
    pub r#sas_token: Box<Option<String>>,
    /// A `service_principal` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "servicePrincipal")]
    pub r#service_principal: Box<Option<super::super::types::arckubernetes::FluxConfigurationBlobStorageServicePrincipal>>,
    /// Specifies the interval at which to re-reconcile the cluster Azure Blob source with the remote.
    #[builder(into, default)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Box<Option<i32>>,
    /// Specifies the maximum time to attempt to reconcile the cluster Azure Blob source with the remote.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
}
