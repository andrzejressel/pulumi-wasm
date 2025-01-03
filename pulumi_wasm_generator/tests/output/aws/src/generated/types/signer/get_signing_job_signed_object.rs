#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSigningJobSignedObject {
    #[builder(into)]
    #[serde(rename = "s3s")]
    pub r#s_3_s: Box<Vec<super::super::types::signer::GetSigningJobSignedObjectS3>>,
}
