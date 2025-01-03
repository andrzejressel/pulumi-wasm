#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetDomainJoinInfo {
    /// Fully qualified name of the directory (for example, corp.example.com).
    #[builder(into, default)]
    #[serde(rename = "directoryName")]
    pub r#directory_name: Box<Option<String>>,
    /// Distinguished name of the organizational unit for computer accounts.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitDistinguishedName")]
    pub r#organizational_unit_distinguished_name: Box<Option<String>>,
}
