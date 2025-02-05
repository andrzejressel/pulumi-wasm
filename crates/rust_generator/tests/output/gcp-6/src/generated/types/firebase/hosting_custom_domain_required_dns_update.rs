#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingCustomDomainRequiredDnsUpdate {
    /// (Output)
    /// The last time Hosting checked your CustomDomain's DNS records.
    #[builder(into, default)]
    #[serde(rename = "checkTime")]
    pub r#check_time: Box<Option<String>>,
    /// A text string to serve at the path.
    #[builder(into, default)]
    #[serde(rename = "desireds")]
    pub r#desireds: Box<Option<Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdateDesired>>>,
    /// Whether Hosting was able to find the required file contents on the
    /// specified path during its last check.
    #[builder(into, default)]
    #[serde(rename = "discovereds")]
    pub r#discovereds: Box<Option<Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdateDiscovered>>>,
}
