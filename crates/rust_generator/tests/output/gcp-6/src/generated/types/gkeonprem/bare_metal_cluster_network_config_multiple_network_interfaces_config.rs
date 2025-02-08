#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterNetworkConfigMultipleNetworkInterfacesConfig {
    /// Whether to enable multiple network interfaces for your pods.
    /// When set network_config.advanced_networking is automatically
    /// set to true.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
