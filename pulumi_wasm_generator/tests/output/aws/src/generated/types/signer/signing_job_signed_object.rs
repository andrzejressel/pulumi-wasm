#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SigningJobSignedObject {
    #[builder(into, default)]
    #[serde(rename = "s3s")]
    pub r#s_3_s: Box<Option<Vec<super::super::types::signer::SigningJobSignedObjectS3>>>,
}