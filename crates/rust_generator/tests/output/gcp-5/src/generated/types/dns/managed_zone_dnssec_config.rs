#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagedZoneDnssecConfig {
    /// Specifies parameters that will be used for generating initial DnsKeys
    /// for this ManagedZone. If you provide a spec for keySigning or zoneSigning,
    /// you must also provide one for the other.
    /// default_key_specs can only be updated when the state is `off`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "defaultKeySpecs")]
    pub r#default_key_specs: Box<Option<Vec<super::super::types::dns::ManagedZoneDnssecConfigDefaultKeySpec>>>,
    /// Identifies what kind of resource this is
    #[builder(into, default)]
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// Specifies the mechanism used to provide authenticated denial-of-existence responses.
    /// non_existence can only be updated when the state is `off`.
    /// Possible values are: `nsec`, `nsec3`.
    #[builder(into, default)]
    #[serde(rename = "nonExistence")]
    pub r#non_existence: Box<Option<String>>,
    /// Specifies whether DNSSEC is enabled, and what mode it is in
    /// Possible values are: `off`, `on`, `transfer`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
