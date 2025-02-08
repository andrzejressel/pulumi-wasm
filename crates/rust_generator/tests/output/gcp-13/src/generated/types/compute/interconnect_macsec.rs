#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InterconnectMacsec {
    /// If set to true, the Interconnect connection is configured with a should-secure
    /// MACsec security policy, that allows the Google router to fallback to cleartext
    /// traffic if the MKA session cannot be established. By default, the Interconnect
    /// connection is configured with a must-secure security policy that drops all traffic
    /// if the MKA session cannot be established with your router.
    #[builder(into, default)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// A keychain placeholder describing a set of named key objects along with their
    /// start times. A MACsec CKN/CAK is generated for each key in the key chain.
    /// Google router automatically picks the key with the most recent startTime when establishing
    /// or re-establishing a MACsec secure link.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preSharedKeys")]
    pub r#pre_shared_keys: Box<Vec<super::super::types::compute::InterconnectMacsecPreSharedKey>>,
}
