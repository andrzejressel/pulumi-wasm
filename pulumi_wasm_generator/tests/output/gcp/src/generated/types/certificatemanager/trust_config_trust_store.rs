#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TrustConfigTrustStore {
    /// Set of intermediate CA certificates used for the path building phase of chain validation.
    /// The field is currently not supported if trust config is used for the workload certificate feature.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "intermediateCas")]
    pub r#intermediate_cas: Box<Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStoreIntermediateCa>>>,
    /// List of Trust Anchors to be used while performing validation against a given TrustStore.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "trustAnchors")]
    pub r#trust_anchors: Box<Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStoreTrustAnchor>>>,
}
