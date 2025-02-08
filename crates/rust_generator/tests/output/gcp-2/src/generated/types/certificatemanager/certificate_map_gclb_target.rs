#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateMapGclbTarget {
    /// An IP configuration where this Certificate Map is serving
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ipConfigs")]
    pub r#ip_configs: Box<Option<Vec<super::super::types::certificatemanager::CertificateMapGclbTargetIpConfig>>>,
    /// Proxy name must be in the format projects/*/locations/*/targetHttpsProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into, default)]
    #[serde(rename = "targetHttpsProxy")]
    pub r#target_https_proxy: Box<Option<String>>,
    /// Proxy name must be in the format projects/*/locations/*/targetSslProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into, default)]
    #[serde(rename = "targetSslProxy")]
    pub r#target_ssl_proxy: Box<Option<String>>,
}
