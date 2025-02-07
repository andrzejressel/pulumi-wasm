#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxWebAppSlotLogsApplicationLogsAzureBlobStorage {
    /// The level at which to log. Possible values include `Error`, `Warning`, `Information`, `Verbose` and `Off`. **NOTE:** this field is not available for `http_logs`
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Box<String>,
    /// The time in days after which to remove blobs. A value of `0` means no retention.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Box<i32>,
    /// SAS URL to an Azure blob container with read/write/list/delete permissions.
    #[builder(into)]
    #[serde(rename = "sasUrl")]
    pub r#sas_url: Box<String>,
}
