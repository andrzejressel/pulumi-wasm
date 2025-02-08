#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TransferJobTransferSpecAzureBlobStorageDataSourceAzureCredentials {
    /// Azure shared access signature. See [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview).
    /// 
    /// <a name="nested_schedule_start_end_date"></a>The `schedule_start_date` and `schedule_end_date` blocks support:
    #[builder(into)]
    #[serde(rename = "sasToken")]
    pub r#sas_token: Box<String>,
}
