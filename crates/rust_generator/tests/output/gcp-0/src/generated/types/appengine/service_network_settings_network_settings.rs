#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceNetworkSettingsNetworkSettings {
    /// The ingress settings for version or service.
    /// Default value is `INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED`.
    /// Possible values are: `INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED`, `INGRESS_TRAFFIC_ALLOWED_ALL`, `INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY`, `INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB`.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "ingressTrafficAllowed")]
    pub r#ingress_traffic_allowed: Box<Option<String>>,
}
