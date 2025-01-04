#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCaaRecordRecord {
    /// Extensible CAA flags, currently only 1 is implemented to set the issuer critical flag.
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Box<i32>,
    /// A property tag, options are `issue`, `issuewild` and `iodef`.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// A property value such as a registrar domain.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
