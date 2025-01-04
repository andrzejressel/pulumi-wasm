#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitCluster {
    /// Specifies the Azure blob service endpoint, for example, `core.windows.net`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "azureServiceEndpoint")]
    pub r#azure_service_endpoint: Box<String>,
    /// Specifies the Azure Storage account name of the cloud witness for the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "cloudAccountName")]
    pub r#cloud_account_name: Box<String>,
    /// Specifies the name of the cluster. It must be 3-15 characters long and contain only letters, numbers and hyphens. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the fileshare path of the local witness for the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "witnessPath")]
    pub r#witness_path: Box<String>,
    /// Specifies the type of the witness. Possible values are `Cloud`, `FileShare`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "witnessType")]
    pub r#witness_type: Box<String>,
}
