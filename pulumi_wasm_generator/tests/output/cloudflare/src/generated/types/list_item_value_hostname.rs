#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListItemValueHostname {
    /// The FQDN to match on. Wildcard sub-domain matching is allowed. Eg. *.abc.com.
    #[builder(into)]
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
