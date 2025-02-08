#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagedZoneDnssecConfigDefaultKeySpec {
    /// String mnemonic specifying the DNSSEC algorithm of this key
    /// Possible values are: `ecdsap256sha256`, `ecdsap384sha384`, `rsasha1`, `rsasha256`, `rsasha512`.
    #[builder(into, default)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<String>>,
    /// Length of the keys in bits
    #[builder(into, default)]
    #[serde(rename = "keyLength")]
    pub r#key_length: Box<Option<i32>>,
    /// Specifies whether this is a key signing key (KSK) or a zone
    /// signing key (ZSK). Key signing keys have the Secure Entry
    /// Point flag set and, when active, will only be used to sign
    /// resource record sets of type DNSKEY. Zone signing keys do
    /// not have the Secure Entry Point flag set and will be used
    /// to sign all other types of resource record sets.
    /// Possible values are: `keySigning`, `zoneSigning`.
    #[builder(into, default)]
    #[serde(rename = "keyType")]
    pub r#key_type: Box<Option<String>>,
    /// Identifies what kind of resource this is
    #[builder(into, default)]
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
}
