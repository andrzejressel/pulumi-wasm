#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResponsePolicyRuleLocalDataLocalData {
    /// For example, www.example.com.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)
    #[builder(into, default)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Box<Option<Vec<String>>>,
    /// Number of seconds that this ResourceRecordSet can be cached by
    /// resolvers.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
    /// One of valid DNS resource types.
    /// Possible values are: `A`, `AAAA`, `CAA`, `CNAME`, `DNSKEY`, `DS`, `HTTPS`, `IPSECVPNKEY`, `MX`, `NAPTR`, `NS`, `PTR`, `SOA`, `SPF`, `SRV`, `SSHFP`, `SVCB`, `TLSA`, `TXT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
