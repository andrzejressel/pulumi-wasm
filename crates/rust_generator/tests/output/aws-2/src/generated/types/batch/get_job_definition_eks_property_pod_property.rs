#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionEksPropertyPodProperty {
    /// The properties of the container that's used on the Amazon EKS pod. See containers below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyContainer>>,
    /// The DNS policy for the pod. The default value is ClusterFirst. If the hostNetwork parameter is not specified, the default is ClusterFirstWithHostNet. ClusterFirst indicates that any DNS query that does not match the configured cluster domain suffix is forwarded to the upstream nameserver inherited from the node.
    #[builder(into)]
    #[serde(rename = "dnsPolicy")]
    pub r#dns_policy: Box<String>,
    /// Indicates if the pod uses the hosts' network IP address. The default value is true. Setting this to false enables the Kubernetes pod networking model. Most AWS Batch workloads are egress-only and don't require the overhead of IP allocation for each pod for incoming connections.
    #[builder(into)]
    #[serde(rename = "hostNetwork")]
    pub r#host_network: Box<bool>,
    #[builder(into)]
    #[serde(rename = "imagePullSecrets")]
    pub r#image_pull_secrets: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyImagePullSecret>>,
    /// Containers which run before application containers, always runs to completion, and must complete successfully before the next container starts. These containers are registered with the Amazon EKS Connector agent and persists the registration information in the Kubernetes backend data store. See containers below.
    #[builder(into)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyInitContainer>>,
    /// Metadata about the Kubernetes pod.
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyMetadata>>,
    /// The name of the service account that's used to run the pod.
    #[builder(into)]
    #[serde(rename = "serviceAccountName")]
    pub r#service_account_name: Box<String>,
    /// (Optional) Indicates if the processes in a container are shared, or visible, to other containers in the same pod.
    #[builder(into)]
    #[serde(rename = "shareProcessNamespace")]
    pub r#share_process_namespace: Box<bool>,
    /// A list of data volumes used in a job.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolume>>,
}
