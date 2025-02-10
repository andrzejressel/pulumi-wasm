/// Cluster contains information about a Google Distributed Cloud Edge Kubernetes cluster.
///
///
/// To get more information about Cluster, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/container/rest/v1/projects.locations.clusters)
/// * How-to Guides
///     * [Create and manage clusters](https://cloud.google.com/distributed-cloud/edge/latest/docs/clusters)
///
///
///
/// ## Example Usage
///
/// ### Edgecontainer Cluster
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: basic-cluster
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
///       labels:
///         my_key: my_val
///         other_key: other_val
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Edgecontainer Cluster With Maintenance Window
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: cluster-with-maintenance
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
///       maintenancePolicy:
///         window:
///           recurringWindow:
///             window:
///               startTime: 2023-01-01T08:00:00Z
///               endTime: 2023-01-01T17:00:00Z
///             recurrence: FREQ=WEEKLY;BYDAY=SA
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Edgecontainer Local Control Plane Cluster
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: local-control-plane-cluster
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
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/cluster:Cluster default projects/{{project}}/locations/{{location}}/clusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/cluster:Cluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/cluster:Cluster default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// RBAC policy that will be applied and managed by GEC.
        /// Structure is documented below.
        #[builder(into)]
        pub authorization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::edgecontainer::ClusterAuthorization,
        >,
        /// The configuration of the cluster control plane.
        #[builder(into, default)]
        pub control_plane: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::ClusterControlPlane>,
        >,
        /// Remote control plane disk encryption options. This field is only used when enabling CMEK support.
        #[builder(into, default)]
        pub control_plane_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::ClusterControlPlaneEncryption>,
        >,
        /// The default maximum number of pods per node used if a maximum value is not specified explicitly for a node pool in this
        /// cluster. If unspecified, the Kubernetes default value will be used.
        #[builder(into, default)]
        pub default_max_pods_per_node: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Address pools for cluster data plane external load balancing.
        #[builder(into, default)]
        pub external_load_balancer_ipv4_address_pools: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Fleet related configuration.
        /// Fleets are a Google Cloud concept for logically organizing clusters,
        /// letting you use and manage multi-cluster capabilities and apply
        /// consistent policies across your systems.
        /// Structure is documented below.
        #[builder(into)]
        pub fleet: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::edgecontainer::ClusterFleet,
        >,
        /// User-defined labels for the edgecloud cluster. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Cluster-wide maintenance policy configuration.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::ClusterMaintenancePolicy>,
        >,
        /// The GDCE cluster name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Fleet related configuration.
        /// Fleets are a Google Cloud concept for logically organizing clusters,
        /// letting you use and manage multi-cluster capabilities and apply
        /// consistent policies across your systems.
        /// Structure is documented below.
        #[builder(into)]
        pub networking: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::edgecontainer::ClusterNetworking,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The release channel a cluster is subscribed to. Possible values: ["RELEASE_CHANNEL_UNSPECIFIED", "NONE", "REGULAR"]
        #[builder(into, default)]
        pub release_channel: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Config that customers are allowed to define for GDCE system add-ons.
        #[builder(into, default)]
        pub system_addons_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::edgecontainer::ClusterSystemAddonsConfig>,
        >,
        /// (Output)
        /// The target version of the cluster.
        #[builder(into, default)]
        pub target_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// RBAC policy that will be applied and managed by GEC.
        /// Structure is documented below.
        pub authorization: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterAuthorization,
        >,
        /// The PEM-encoded public certificate of the cluster's CA.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub cluster_ca_certificate: pulumi_gestalt_rust::Output<String>,
        /// The configuration of the cluster control plane.
        pub control_plane: pulumi_gestalt_rust::Output<
            Option<super::super::types::edgecontainer::ClusterControlPlane>,
        >,
        /// Remote control plane disk encryption options. This field is only used when enabling CMEK support.
        pub control_plane_encryption: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterControlPlaneEncryption,
        >,
        /// The control plane release version.
        pub control_plane_version: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// The time when the maintenance event request was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The default maximum number of pods per node used if a maximum value is not specified explicitly for a node pool in this
        /// cluster. If unspecified, the Kubernetes default value will be used.
        pub default_max_pods_per_node: pulumi_gestalt_rust::Output<i32>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP address of the Kubernetes API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Address pools for cluster data plane external load balancing.
        pub external_load_balancer_ipv4_address_pools: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Fleet related configuration.
        /// Fleets are a Google Cloud concept for logically organizing clusters,
        /// letting you use and manage multi-cluster capabilities and apply
        /// consistent policies across your systems.
        /// Structure is documented below.
        pub fleet: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterFleet,
        >,
        /// User-defined labels for the edgecloud cluster. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// All the maintenance events scheduled for the cluster, including the ones
        /// ongoing, planned for the future and done in the past (up to 90 days).
        /// Structure is documented below.
        pub maintenance_events: pulumi_gestalt_rust::Output<
            Vec<super::super::types::edgecontainer::ClusterMaintenanceEvent>,
        >,
        /// Cluster-wide maintenance policy configuration.
        pub maintenance_policy: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterMaintenancePolicy,
        >,
        /// The GDCE cluster name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Fleet related configuration.
        /// Fleets are a Google Cloud concept for logically organizing clusters,
        /// letting you use and manage multi-cluster capabilities and apply
        /// consistent policies across your systems.
        /// Structure is documented below.
        pub networking: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterNetworking,
        >,
        /// The lowest release version among all worker nodes. This field can be empty
        /// if the cluster does not have any worker nodes.
        pub node_version: pulumi_gestalt_rust::Output<String>,
        /// The port number of the Kubernetes API server.
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The release channel a cluster is subscribed to. Possible values: ["RELEASE_CHANNEL_UNSPECIFIED", "NONE", "REGULAR"]
        pub release_channel: pulumi_gestalt_rust::Output<String>,
        /// Indicates the status of the cluster.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Config that customers are allowed to define for GDCE system add-ons.
        pub system_addons_config: pulumi_gestalt_rust::Output<
            super::super::types::edgecontainer::ClusterSystemAddonsConfig,
        >,
        /// (Output)
        /// The target version of the cluster.
        pub target_version: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// The time when the maintenance event message was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorization_binding = args.authorization.get_output(context);
        let control_plane_binding = args.control_plane.get_output(context);
        let control_plane_encryption_binding = args
            .control_plane_encryption
            .get_output(context);
        let default_max_pods_per_node_binding = args
            .default_max_pods_per_node
            .get_output(context);
        let external_load_balancer_ipv4_address_pools_binding = args
            .external_load_balancer_ipv4_address_pools
            .get_output(context);
        let fleet_binding = args.fleet.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_policy_binding = args.maintenance_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let networking_binding = args.networking.get_output(context);
        let project_binding = args.project.get_output(context);
        let release_channel_binding = args.release_channel.get_output(context);
        let system_addons_config_binding = args.system_addons_config.get_output(context);
        let target_version_binding = args.target_version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:edgecontainer/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorization".into(),
                    value: authorization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlane".into(),
                    value: control_plane_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneEncryption".into(),
                    value: control_plane_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultMaxPodsPerNode".into(),
                    value: default_max_pods_per_node_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalLoadBalancerIpv4AddressPools".into(),
                    value: external_load_balancer_ipv4_address_pools_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleet".into(),
                    value: fleet_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenancePolicy".into(),
                    value: maintenance_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networking".into(),
                    value: networking_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseChannel".into(),
                    value: release_channel_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "systemAddonsConfig".into(),
                    value: system_addons_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVersion".into(),
                    value: target_version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            authorization: o.get_field("authorization"),
            cluster_ca_certificate: o.get_field("clusterCaCertificate"),
            control_plane: o.get_field("controlPlane"),
            control_plane_encryption: o.get_field("controlPlaneEncryption"),
            control_plane_version: o.get_field("controlPlaneVersion"),
            create_time: o.get_field("createTime"),
            default_max_pods_per_node: o.get_field("defaultMaxPodsPerNode"),
            effective_labels: o.get_field("effectiveLabels"),
            endpoint: o.get_field("endpoint"),
            external_load_balancer_ipv4_address_pools: o
                .get_field("externalLoadBalancerIpv4AddressPools"),
            fleet: o.get_field("fleet"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maintenance_events: o.get_field("maintenanceEvents"),
            maintenance_policy: o.get_field("maintenancePolicy"),
            name: o.get_field("name"),
            networking: o.get_field("networking"),
            node_version: o.get_field("nodeVersion"),
            port: o.get_field("port"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            release_channel: o.get_field("releaseChannel"),
            status: o.get_field("status"),
            system_addons_config: o.get_field("systemAddonsConfig"),
            target_version: o.get_field("targetVersion"),
            update_time: o.get_field("updateTime"),
        }
    }
}
