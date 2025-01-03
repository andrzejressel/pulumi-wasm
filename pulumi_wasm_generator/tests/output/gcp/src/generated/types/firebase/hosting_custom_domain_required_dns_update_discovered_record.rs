#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingCustomDomainRequiredDnsUpdateDiscoveredRecord {
    /// The domain name the record pertains to, e.g. `foo.bar.com.`.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    /// The data of the record. The meaning of the value depends on record type:
    /// - A and AAAA: IP addresses for the domain name.
    /// - CNAME: Another domain to check for records.
    /// - TXT: Arbitrary text strings associated with the domain name. Hosting
    /// uses TXT records to determine a which Firebase Projects have
    /// permission to act on the domain name's behalf.
    /// - CAA: The record's flags, tag, and value, e.g. `0 issue "pki.goog"`.
    #[builder(into, default)]
    #[serde(rename = "rdata")]
    pub r#rdata: Box<Option<String>>,
    /// Indicates the a required action for this record.
    #[builder(into, default)]
    #[serde(rename = "requiredAction")]
    pub r#required_action: Box<Option<String>>,
    /// The record's type, which determines what data the record contains.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
