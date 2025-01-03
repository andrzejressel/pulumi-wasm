#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificateDescriptionX509DescriptionKeyUsageUnknownExtendedKeyUsage {
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    #[builder(into, default)]
    #[serde(rename = "objectIdPaths")]
    pub r#object_id_paths: Box<Option<Vec<i32>>>,
}
