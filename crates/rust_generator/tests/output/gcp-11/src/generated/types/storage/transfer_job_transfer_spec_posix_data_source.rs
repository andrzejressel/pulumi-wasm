#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobTransferSpecPosixDataSource {
    /// Root directory path to the filesystem.
    #[builder(into)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: Box<String>,
}
