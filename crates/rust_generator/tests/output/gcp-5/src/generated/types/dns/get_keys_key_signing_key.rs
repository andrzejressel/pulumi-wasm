#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetKeysKeySigningKey {
    /// String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time. Possible values are `ecdsap256sha256`, `ecdsap384sha384`, `rsasha1`, `rsasha256`, and `rsasha512`.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<String>,
    /// The time that this resource was created in the control plane. This is in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Box<String>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// A list of cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Each contains:
    #[builder(into)]
    #[serde(rename = "digests")]
    pub r#digests: Box<Vec<super::super::types::dns::GetKeysKeySigningKeyDigest>>,
    /// The DS record based on the KSK record. This is used when [delegating](https://cloud.google.com/dns/docs/dnssec-advanced#subdelegation) DNSSEC-signed subdomains.
    #[builder(into)]
    #[serde(rename = "dsRecord")]
    pub r#ds_record: Box<String>,
    /// Unique identifier for the resource; defined by the server.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Active keys will be used to sign subsequent changes to the ManagedZone. Inactive keys will still be present as DNSKEY Resource Records for the use of resolvers validating existing signatures.
    #[builder(into)]
    #[serde(rename = "isActive")]
    pub r#is_active: Box<bool>,
    /// Length of the key in bits. Specified at creation time then immutable.
    #[builder(into)]
    #[serde(rename = "keyLength")]
    pub r#key_length: Box<i32>,
    /// The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B.
    #[builder(into)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Box<i32>,
    /// Base64 encoded public half of this key.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
