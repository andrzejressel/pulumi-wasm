#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRepositoryImageScanningConfiguration {
    /// Whether images are scanned after being pushed to the repository.
    #[builder(into)]
    #[serde(rename = "scanOnPush")]
    pub r#scan_on_push: Box<bool>,
}
