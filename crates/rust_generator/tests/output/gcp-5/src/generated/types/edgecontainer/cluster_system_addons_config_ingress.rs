#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterSystemAddonsConfigIngress {
    /// Whether Ingress is disabled.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Ingress VIP.
    #[builder(into, default)]
    #[serde(rename = "ipv4Vip")]
    pub r#ipv_4_vip: Box<Option<String>>,
}
