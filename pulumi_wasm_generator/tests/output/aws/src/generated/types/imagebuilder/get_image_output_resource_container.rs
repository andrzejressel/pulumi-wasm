#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageOutputResourceContainer {
    /// Set of URIs for created containers.
    #[builder(into)]
    #[serde(rename = "imageUris")]
    pub r#image_uris: Box<Vec<String>>,
    /// Region of the container image.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}