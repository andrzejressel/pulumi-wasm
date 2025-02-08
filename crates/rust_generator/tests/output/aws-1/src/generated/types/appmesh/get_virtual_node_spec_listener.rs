#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVirtualNodeSpecListener {
    #[builder(into)]
    #[serde(rename = "connectionPools")]
    pub r#connection_pools: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPool>>,
    #[builder(into)]
    #[serde(rename = "healthChecks")]
    pub r#health_checks: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerHealthCheck>>,
    #[builder(into)]
    #[serde(rename = "outlierDetections")]
    pub r#outlier_detections: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetection>>,
    #[builder(into)]
    #[serde(rename = "portMappings")]
    pub r#port_mappings: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerPortMapping>>,
    #[builder(into)]
    #[serde(rename = "timeouts")]
    pub r#timeouts: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeout>>,
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTl>>,
}
