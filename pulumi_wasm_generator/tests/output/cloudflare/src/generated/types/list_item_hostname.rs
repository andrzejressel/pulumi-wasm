#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListItemHostname {
    /// The FQDN to match on.
    #[builder(into)]
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
