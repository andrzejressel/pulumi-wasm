#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageOutputResource {
    /// Set of objects with each Amazon Machine Image (AMI) created.
    #[builder(into, default)]
    #[serde(rename = "amis")]
    pub r#amis: Box<Option<Vec<super::super::types::imagebuilder::ImageOutputResourceAmi>>>,
    /// Set of objects with each container image created and stored in the output repository.
    #[builder(into, default)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Option<Vec<super::super::types::imagebuilder::ImageOutputResourceContainer>>>,
}