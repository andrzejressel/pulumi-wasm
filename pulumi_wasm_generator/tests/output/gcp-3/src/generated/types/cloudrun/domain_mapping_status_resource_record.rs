#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainMappingStatusResourceRecord {
    /// Name should be a [verified](https://support.google.com/webmasters/answer/9008080) domain
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// (Output)
    /// Data for this record. Values vary by record type, as defined in RFC 1035
    /// (section 5) and RFC 1034 (section 3.6.1).
    #[builder(into, default)]
    #[serde(rename = "rrdata")]
    pub r#rrdata: Box<Option<String>>,
    /// Resource record type. Example: `AAAA`.
    /// Possible values are: `A`, `AAAA`, `CNAME`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
