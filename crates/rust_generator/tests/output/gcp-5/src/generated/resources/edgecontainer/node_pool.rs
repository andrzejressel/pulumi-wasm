/// "A set of Kubernetes nodes in a cluster with common configuration and specification."
///
///
/// To get more information about NodePool, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/container/rest/v1/projects.locations.clusters.nodePools)
/// * How-to Guides
///     * [Google Distributed Cloud Edge](https://cloud.google.com/distributed-cloud/edge/latest/docs)
///
/// ## Example Usage
///
/// ### Edgecontainer Node Pool
///
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: default
///       location: us-central1
///       authorization:
///         adminUsers:
///           username: admin@hashicorptest.com
///       networking:
///         clusterIpv4CidrBlocks:
///           - 10.0.0.0/16
///         servicesIpv4CidrBlocks:
///           - 10.1.0.0/16
///       fleet:
///         project: projects/${project.number}
///   default:
///     type: gcp:edgecontainer:NodePool
///     properties:
///       name: nodepool-1
///       cluster: ${cluster.name}
///       location: us-central1
///       nodeLocation: us-central1-edge-example-edgesite
///       nodeCount: 3
///       labels:
///         my_key: my_val
///         other_key: other_val
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Edgecontainer Node Pool With Cmek
///
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: default
///       location: us-central1
///       authorization:
///         adminUsers:
///           username: admin@hashicorptest.com
///       networking:
///         clusterIpv4CidrBlocks:
///           - 10.0.0.0/16
///         servicesIpv4CidrBlocks:
///           - 10.1.0.0/16
///       fleet:
///         project: projects/${project.number}
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: ${cryptoKeyCryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-edgecontainer.iam.gserviceaccount.com
///   cryptoKeyCryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: key
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: keyring
///       location: us-central1
///   default:
///     type: gcp:edgecontainer:NodePool
///     properties:
///       name: nodepool-1
///       cluster: ${cluster.name}
///       location: us-central1
///       nodeLocation: us-central1-edge-example-edgesite
///       nodeCount: 3
///       localDiskEncryption:
///         kmsKey: ${cryptoKeyCryptoKey.id}
///     options:
///       dependsOn:
///         - ${cryptoKey}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Edgecontainer Local Control Plane Node Pool
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: ""
///       location: us-central1
///       authorization:
///         adminUsers:
///           username: admin@hashicorptest.com
///       networking:
///         clusterIpv4CidrBlocks:
///           - 10.0.0.0/16
///         servicesIpv4CidrBlocks:
///           - 10.1.0.0/16
///       fleet:
///         project: projects/${project.number}
///       externalLoadBalancerIpv4AddressPools:
///         - 10.100.0.0-10.100.0.10
///       controlPlane:
///         local:
///           nodeLocation: us-central1-edge-example-edgesite
///           nodeCount: 1
///           machineFilter: machine-name
///           sharedDeploymentPolicy: ALLOWED
///   defaultNodePool:
///     type: gcp:edgecontainer:NodePool
///     name: default
///     properties:
///       name: nodepool-1
///       cluster: ${cluster.name}
///       location: us-central1
///       nodeLocation: us-central1-edge-example-edgesite
///       nodeCount: 3
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// NodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/nodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{name}}`
///
/// * `{{location}}/{{cluster}}/{{name}}`
///
/// When using the `pulumi import` command, NodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/nodePool:NodePool default projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/nodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/nodePool:NodePool default {{project}}/{{location}}/{{cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/nodePool:NodePool default {{location}}/{{cluster}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodePoolArgs {
        /// The name of the target Distributed Cloud Edge Cluster.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Local disk encryption options. This field is only used when enabling CMEK support.
        /// Structure is documented below.
        #[builder(into, default)]
        pub local_disk_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::NodePoolLocalDiskEncryption>,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only machines matching this filter will be allowed to join the node pool.
        /// The filtering language accepts strings like "name=<name>", and is
        /// documented in more detail in [AIP-160](https://google.aip.dev/160).
        #[builder(into, default)]
        pub machine_filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the node pool.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for each node in the NodePool
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::NodePoolNodeConfig>,
        >,
        /// The number of nodes in the pool.
        #[builder(into)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Name of the Google Distributed Cloud Edge zone where this node pool will be created. For example: `us-central1-edge-customer-a`.
        #[builder(into)]
        pub node_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodePoolResult {
        /// The name of the target Distributed Cloud Edge Cluster.
        ///
        ///
        /// - - -
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// The time when the node pool was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Local disk encryption options. This field is only used when enabling CMEK support.
        /// Structure is documented below.
        pub local_disk_encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::edgecontainer::NodePoolLocalDiskEncryption>,
        >,
        /// The location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Only machines matching this filter will be allowed to join the node pool.
        /// The filtering language accepts strings like "name=<name>", and is
        /// documented in more detail in [AIP-160](https://google.aip.dev/160).
        pub machine_filter: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the node pool.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for each node in the NodePool
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::NodePoolNodeConfig,
        >,
        /// The number of nodes in the pool.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// Name of the Google Distributed Cloud Edge zone where this node pool will be created. For example: `us-central1-edge-customer-a`.
        pub node_location: pulumi_gestalt_rust::Output<String>,
        /// The lowest release version among all worker nodes.
        pub node_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time when the node pool was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NodePoolArgs,
    ) -> NodePoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_binding = args.cluster.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let local_disk_encryption_binding = args
            .local_disk_encryption
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let machine_filter_binding = args.machine_filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let node_count_binding = args.node_count.get_output(context);
        let node_location_binding = args.node_location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:edgecontainer/nodePool:NodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localDiskEncryption".into(),
                    value: &local_disk_encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineFilter".into(),
                    value: &machine_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeLocation".into(),
                    value: &node_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NodePoolResult {
            cluster: o.get_field("cluster"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            local_disk_encryption: o.get_field("localDiskEncryption"),
            location: o.get_field("location"),
            machine_filter: o.get_field("machineFilter"),
            name: o.get_field("name"),
            node_config: o.get_field("nodeConfig"),
            node_count: o.get_field("nodeCount"),
            node_location: o.get_field("nodeLocation"),
            node_version: o.get_field("nodeVersion"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            update_time: o.get_field("updateTime"),
        }
    }
}
