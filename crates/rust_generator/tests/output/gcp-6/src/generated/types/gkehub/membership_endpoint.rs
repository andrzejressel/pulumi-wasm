#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MembershipEndpoint {
    /// If this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gkeCluster")]
    pub r#gke_cluster: Box<Option<super::super::types::gkehub::MembershipEndpointGkeCluster>>,
}
