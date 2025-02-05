#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventHubCaptureDescriptionDestination {
    /// The Blob naming convention for archiving. e.g. `{Namespace}/{EventHub}/{PartitionId}/{Year}/{Month}/{Day}/{Hour}/{Minute}/{Second}`. Here all the parameters (Namespace,EventHub .. etc) are mandatory irrespective of order
    #[builder(into)]
    #[serde(rename = "archiveNameFormat")]
    pub r#archive_name_format: Box<String>,
    /// The name of the Container within the Blob Storage Account where messages should be archived.
    #[builder(into)]
    #[serde(rename = "blobContainerName")]
    pub r#blob_container_name: Box<String>,
    /// The Name of the Destination where the capture should take place. At this time the only supported value is `EventHubArchive.AzureBlockBlob`.
    /// 
    /// > At this time it's only possible to Capture EventHub messages to Blob Storage. There's [a Feature Request for the Azure SDK to add support for Capturing messages to Azure Data Lake here](https://github.com/Azure/azure-rest-api-specs/issues/2255).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the Blob Storage Account where messages should be archived.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<String>,
}
