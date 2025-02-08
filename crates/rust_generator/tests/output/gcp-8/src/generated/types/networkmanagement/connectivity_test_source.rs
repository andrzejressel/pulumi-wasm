#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectivityTestSource {
    /// A Compute Engine instance URI.
    #[builder(into, default)]
    #[serde(rename = "instance")]
    pub r#instance: Box<Option<String>>,
    /// The IP address of the endpoint, which can be an external or
    /// internal IP. An IPv6 address is only allowed when the test's
    /// destination is a global load balancer VIP.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// A Compute Engine network URI.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// Type of the network where the endpoint is located.
    /// Possible values are: `GCP_NETWORK`, `NON_GCP_NETWORK`.
    #[builder(into, default)]
    #[serde(rename = "networkType")]
    pub r#network_type: Box<Option<String>>,
    /// The IP protocol port of the endpoint. Only applicable when
    /// protocol is TCP or UDP.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Project ID where the endpoint is located. The Project ID can be
    /// derived from the URI if you provide a VM instance or network URI.
    /// The following are two cases where you must provide the project ID:
    /// 1. Only the IP address is specified, and the IP address is
    /// within a GCP project.
    /// 2. When you are using Shared VPC and the IP address
    /// that you provide is from the service project. In this case,
    /// the network that the IP address resides in is defined in the
    /// host project.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
}
