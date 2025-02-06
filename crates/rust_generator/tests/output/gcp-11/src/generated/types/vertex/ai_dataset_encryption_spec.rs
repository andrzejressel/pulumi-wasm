#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiDatasetEncryptionSpec {
    /// Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource.
    /// Has the form: projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key. The key needs to be in the same region as where the resource is created.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
}
