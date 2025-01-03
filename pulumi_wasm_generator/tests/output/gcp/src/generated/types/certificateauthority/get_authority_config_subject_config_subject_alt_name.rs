#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityConfigSubjectConfigSubjectAltName {
    /// Contains only valid, fully-qualified host names.
    #[builder(into)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Box<Vec<String>>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[builder(into)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Vec<String>>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Vec<String>>,
    /// Contains only valid RFC 3986 URIs.
    #[builder(into)]
    #[serde(rename = "uris")]
    pub r#uris: Box<Vec<String>>,
}
