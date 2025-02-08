#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VMwareClusterLoadBalancerManualLbConfig {
    /// NodePort for control plane service. The Kubernetes API server in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30968).
    #[builder(into, default)]
    #[serde(rename = "controlPlaneNodePort")]
    pub r#control_plane_node_port: Box<Option<i32>>,
    /// NodePort for ingress service's http. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 32527).
    #[builder(into, default)]
    #[serde(rename = "ingressHttpNodePort")]
    pub r#ingress_http_node_port: Box<Option<i32>>,
    /// NodePort for ingress service's https. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30139).
    #[builder(into, default)]
    #[serde(rename = "ingressHttpsNodePort")]
    pub r#ingress_https_node_port: Box<Option<i32>>,
    /// NodePort for konnectivity server service running as a sidecar in each
    /// kube-apiserver pod (ex. 30564).
    #[builder(into, default)]
    #[serde(rename = "konnectivityServerNodePort")]
    pub r#konnectivity_server_node_port: Box<Option<i32>>,
}
