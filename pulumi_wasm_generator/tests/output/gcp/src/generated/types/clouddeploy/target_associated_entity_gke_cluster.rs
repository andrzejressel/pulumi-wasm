#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetAssociatedEntityGkeCluster {
    /// Optional. Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`.
    #[builder(into, default)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<Option<String>>,
    /// Optional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept).
    #[builder(into, default)]
    #[serde(rename = "internalIp")]
    pub r#internal_ip: Box<Option<bool>>,
    /// Optional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server.
    #[builder(into, default)]
    #[serde(rename = "proxyUrl")]
    pub r#proxy_url: Box<Option<String>>,
}
