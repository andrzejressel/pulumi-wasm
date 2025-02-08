#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AlertRuleFusionSource {
    /// Whether this source signal is enabled or disabled in Fusion detection? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The name of the Fusion source signal. Refer to Fusion alert rule template for supported values.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `sub_type` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "subTypes")]
    pub r#sub_types: Box<Option<Vec<super::super::types::sentinel::AlertRuleFusionSourceSubType>>>,
}
