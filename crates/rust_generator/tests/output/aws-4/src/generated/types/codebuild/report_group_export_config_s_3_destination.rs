#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReportGroupExportConfigS3Destination {
    /// The name of the S3 bucket where the raw data of a report are exported.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// A boolean value that specifies if the results of a report are encrypted.
    /// **Note: the API does not currently allow setting encryption as disabled**
    #[builder(into, default)]
    #[serde(rename = "encryptionDisabled")]
    pub r#encryption_disabled: Box<Option<bool>>,
    /// The encryption key for the report's encrypted raw data. The KMS key ARN.
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<String>,
    /// The type of build output artifact to create. Valid values are: `NONE` (default) and `ZIP`.
    #[builder(into, default)]
    #[serde(rename = "packaging")]
    pub r#packaging: Box<Option<String>>,
    /// The path to the exported report's raw data results.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
