#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EmailServiceDomainVerificationRecordSpf {
    /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Represents an expiry time in seconds to represent how long this entry can be cached by the resolver, default = 3600sec.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
    /// Type of the DNS record. Example: TXT
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// Value of the DNS record.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
