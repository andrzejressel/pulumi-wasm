#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FhirServiceOciArtifact {
    /// A digest of an image within Azure container registry used for export operations of the service instance to narrow the artifacts down.
    #[builder(into, default)]
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    /// An image within Azure container registry used for export operations of the service instance.
    #[builder(into, default)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<Option<String>>,
    /// An Azure container registry used for export operations of the service instance.
    #[builder(into)]
    #[serde(rename = "loginServer")]
    pub r#login_server: Box<String>,
}
