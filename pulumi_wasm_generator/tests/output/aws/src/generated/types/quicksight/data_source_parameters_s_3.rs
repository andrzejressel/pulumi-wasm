#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersS3 {
    /// An object containing the S3 location of the S3 manifest file.
    #[builder(into)]
    #[serde(rename = "manifestFileLocation")]
    pub r#manifest_file_location: Box<super::super::types::quicksight::DataSourceParametersS3ManifestFileLocation>,
}