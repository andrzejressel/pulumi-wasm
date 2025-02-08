#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BareMetalClusterUpgradePolicy {
    /// Specifies which upgrade policy to use.
    /// Possible values are: `SERIAL`, `CONCURRENT`.
    #[builder(into, default)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}
