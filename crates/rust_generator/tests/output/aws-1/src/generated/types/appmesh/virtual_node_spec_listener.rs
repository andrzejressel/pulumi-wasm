#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecListener {
    /// Connection pool information for the listener.
    #[builder(into, default)]
    #[serde(rename = "connectionPool")]
    pub r#connection_pool: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPool>>,
    /// Health check information for the listener.
    #[builder(into, default)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerHealthCheck>>,
    /// Outlier detection information for the listener.
    #[builder(into, default)]
    #[serde(rename = "outlierDetection")]
    pub r#outlier_detection: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerOutlierDetection>>,
    /// Port mapping information for the listener.
    #[builder(into)]
    #[serde(rename = "portMapping")]
    pub r#port_mapping: Box<super::super::types::appmesh::VirtualNodeSpecListenerPortMapping>,
    /// Timeouts for different protocols.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerTimeout>>,
    /// Transport Layer Security (TLS) properties for the listener
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerTls>>,
}
