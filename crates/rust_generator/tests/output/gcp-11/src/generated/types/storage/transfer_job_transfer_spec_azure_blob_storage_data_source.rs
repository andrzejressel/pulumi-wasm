#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TransferJobTransferSpecAzureBlobStorageDataSource {
    /// Credentials used to authenticate API requests to Azure block.
    #[builder(into, default)]
    #[serde(rename = "azureCredentials")]
    pub r#azure_credentials: Box<Option<super::super::types::storage::TransferJobTransferSpecAzureBlobStorageDataSourceAzureCredentials>>,
    /// The container to transfer from the Azure Storage account.`
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Box<String>,
    /// Full Resource name of a secret in Secret Manager containing [SAS Credentials in JSON form](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#azureblobstoragedata:~:text=begin%20with%20a%20%27/%27.-,credentialsSecret,-string). Service Agent for Storage Transfer must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials.`,
    #[builder(into, default)]
    #[serde(rename = "credentialsSecret")]
    pub r#credentials_secret: Box<Option<String>>,
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The name of the Azure Storage account.
    #[builder(into)]
    #[serde(rename = "storageAccount")]
    pub r#storage_account: Box<String>,
}
