#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    CAA,
    MX,
    NAPTR,
    NS,
    PTR,
    SOA,
    SPF,
    SRV,
    TXT,
}
