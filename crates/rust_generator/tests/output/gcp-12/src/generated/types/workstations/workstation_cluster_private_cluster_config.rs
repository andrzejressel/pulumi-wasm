#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkstationClusterPrivateClusterConfig {
    /// Additional project IDs that are allowed to attach to the workstation cluster's service attachment.
    /// By default, the workstation cluster's project and the VPC host project (if different) are allowed.
    #[builder(into, default)]
    #[serde(rename = "allowedProjects")]
    pub r#allowed_projects: Box<Option<Vec<String>>>,
    /// (Output)
    /// Hostname for the workstation cluster.
    /// This field will be populated only when private endpoint is enabled.
    /// To access workstations in the cluster, create a new DNS zone mapping this domain name to an internal IP address and a forwarding rule mapping that address to the service attachment.
    #[builder(into, default)]
    #[serde(rename = "clusterHostname")]
    pub r#cluster_hostname: Box<Option<String>>,
    /// Whether Workstations endpoint is private.
    #[builder(into)]
    #[serde(rename = "enablePrivateEndpoint")]
    pub r#enable_private_endpoint: Box<bool>,
    /// (Output)
    /// Service attachment URI for the workstation cluster.
    /// The service attachment is created when private endpoint is enabled.
    /// To access workstations in the cluster, configure access to the managed service using (Private Service Connect)[https://cloud.google.com/vpc/docs/configure-private-service-connect-services].
    #[builder(into, default)]
    #[serde(rename = "serviceAttachmentUri")]
    pub r#service_attachment_uri: Box<Option<String>>,
}
