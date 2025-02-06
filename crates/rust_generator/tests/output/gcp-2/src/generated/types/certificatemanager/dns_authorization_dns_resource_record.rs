#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DnsAuthorizationDnsResourceRecord {
    /// (Output)
    /// Data of the DNS Resource Record.
    #[builder(into, default)]
    #[serde(rename = "data")]
    pub r#data: Box<Option<String>>,
    /// Name of the resource; provided by the client when the resource is created.
    /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
    /// and all following characters must be a dash, underscore, letter or digit.
    /// 
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// type of DNS authorization. If unset during the resource creation, FIXED_RECORD will
    /// be used for global resources, and PER_PROJECT_RECORD will be used for other locations.
    /// FIXED_RECORD DNS authorization uses DNS-01 validation method
    /// PER_PROJECT_RECORD DNS authorization allows for independent management
    /// of Google-managed certificates with DNS authorization across multiple
    /// projects.
    /// Possible values are: `FIXED_RECORD`, `PER_PROJECT_RECORD`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
