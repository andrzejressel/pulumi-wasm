#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterKubernetesNetworkConfigElasticLoadBalancing {
    /// Indicates if the load balancing capability is enabled on your EKS Auto Mode cluster. If the load balancing capability is enabled, EKS Auto Mode will create and delete load balancers in your Amazon Web Services account.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
