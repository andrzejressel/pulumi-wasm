/// A Google Bare Metal Admin Cluster.
///
///
///
/// ## Example Usage
///
/// ### Gkeonprem Bare Metal Admin Cluster Basic
///
///
/// ```yaml
/// resources:
///   admin-cluster-basic:
///     type: gcp:gkeonprem:BareMetalAdminCluster
///     properties:
///       name: my-cluster
///       location: us-west1
///       bareMetalVersion: 1.13.4
///       networkConfig:
///         islandModeCidr:
///           serviceAddressCidrBlocks:
///             - 172.26.0.0/16
///           podAddressCidrBlocks:
///             - 10.240.0.0/13
///       nodeConfig:
///         maxPodsPerNode: 250
///       controlPlane:
///         controlPlaneNodePoolConfig:
///           nodePoolConfig:
///             labels: {}
///             operatingSystem: LINUX
///             nodeConfigs:
///               - labels: {}
///                 nodeIp: 10.200.0.2
///               - labels: {}
///                 nodeIp: 10.200.0.3
///               - labels: {}
///                 nodeIp: 10.200.0.4
///       loadBalancer:
///         portConfig:
///           controlPlaneLoadBalancerPort: 443
///         vipConfig:
///           controlPlaneVip: 10.200.0.5
///       storage:
///         lvpShareConfig:
///           lvpConfig:
///             path: /mnt/localpv-share
///             storageClass: local-shared
///           sharedPathPvCount: 5
///         lvpNodeMountsConfig:
///           path: /mnt/localpv-disk
///           storageClass: local-disks
///       nodeAccessConfig:
///         loginUser: root
/// ```
/// ### Gkeonprem Bare Metal Admin Cluster Full
///
///
/// ```yaml
/// resources:
///   admin-cluster-basic:
///     type: gcp:gkeonprem:BareMetalAdminCluster
///     properties:
///       name: my-cluster
///       location: us-west1
///       description: test description
///       bareMetalVersion: 1.13.4
///       annotations:
///         env: test
///       networkConfig:
///         islandModeCidr:
///           serviceAddressCidrBlocks:
///             - 172.26.0.0/16
///           podAddressCidrBlocks:
///             - 10.240.0.0/13
///       nodeConfig:
///         maxPodsPerNode: 250
///       controlPlane:
///         controlPlaneNodePoolConfig:
///           nodePoolConfig:
///             labels: {}
///             operatingSystem: LINUX
///             nodeConfigs:
///               - labels: {}
///                 nodeIp: 10.200.0.2
///               - labels: {}
///                 nodeIp: 10.200.0.3
///               - labels: {}
///                 nodeIp: 10.200.0.4
///             taints:
///               - key: test-key
///                 value: test-value
///                 effect: NO_EXECUTE
///         apiServerArgs:
///           - argument: test argument
///             value: test value
///       loadBalancer:
///         portConfig:
///           controlPlaneLoadBalancerPort: 443
///         vipConfig:
///           controlPlaneVip: 10.200.0.5
///         manualLbConfig:
///           enabled: true
///       storage:
///         lvpShareConfig:
///           lvpConfig:
///             path: /mnt/localpv-share
///             storageClass: local-shared
///           sharedPathPvCount: 5
///         lvpNodeMountsConfig:
///           path: /mnt/localpv-disk
///           storageClass: local-disks
///       nodeAccessConfig:
///         loginUser: root
///       securityConfig:
///         authorization:
///           adminUsers:
///             - username: admin@hashicorptest.com
///       maintenanceConfig:
///         maintenanceAddressCidrBlocks:
///           - 10.0.0.1/32
///           - 10.0.0.2/32
///       clusterOperations:
///         enableApplicationLogs: true
///       proxy:
///         uri: test proxy uri
///         noProxies:
///           - 127.0.0.1
/// ```
///
/// ## Import
///
/// BareMetalAdminCluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/bareMetalAdminClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, BareMetalAdminCluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalAdminCluster:BareMetalAdminCluster default projects/{{project}}/locations/{{location}}/bareMetalAdminClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalAdminCluster:BareMetalAdminCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalAdminCluster:BareMetalAdminCluster default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bare_metal_admin_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BareMetalAdminClusterArgs {
        /// Annotations on the Bare Metal Admin Cluster.
        /// This field has the same restrictions as Kubernetes annotations.
        /// The total size of all keys and values combined is limited to 256k.
        /// Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/).
        /// Prefix must be a DNS subdomain.
        /// Name must be 63 characters or less, begin and end with alphanumerics,
        /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A human readable description of this Bare Metal Admin Cluster.
        #[builder(into, default)]
        pub bare_metal_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Admin Cluster's observability infrastructure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cluster_operations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::gkeonprem::BareMetalAdminClusterClusterOperations,
            >,
        >,
        /// Specifies the control plane configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub control_plane: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterControlPlane>,
        >,
        /// A human readable description of this Bare Metal Admin Cluster.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the load balancer configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterLoadBalancer>,
        >,
        /// The location of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the workload node configurations.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::gkeonprem::BareMetalAdminClusterMaintenanceConfig,
            >,
        >,
        /// The bare metal admin cluster name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNetworkConfig>,
        >,
        /// Specifies the node access related settings for the bare metal user cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_access_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNodeAccessConfig>,
        >,
        /// Specifies the workload node configurations.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNodeConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the cluster proxy configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterProxy>,
        >,
        /// Specifies the security related settings for the Bare Metal User Cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub security_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterSecurityConfig>,
        >,
        /// Specifies the cluster storage configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterStorage>,
        >,
    }
    #[allow(dead_code)]
    pub struct BareMetalAdminClusterResult {
        /// Annotations on the Bare Metal Admin Cluster.
        /// This field has the same restrictions as Kubernetes annotations.
        /// The total size of all keys and values combined is limited to 256k.
        /// Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/).
        /// Prefix must be a DNS subdomain.
        /// Name must be 63 characters or less, begin and end with alphanumerics,
        /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A human readable description of this Bare Metal Admin Cluster.
        pub bare_metal_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Admin Cluster's observability infrastructure.
        /// Structure is documented below.
        pub cluster_operations: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::gkeonprem::BareMetalAdminClusterClusterOperations,
            >,
        >,
        /// Specifies the control plane configuration.
        /// Structure is documented below.
        pub control_plane: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterControlPlane>,
        >,
        /// The time the cluster was created, in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was deleted, in RFC3339 text format.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// A human readable description of this Bare Metal Admin Cluster.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP address name of Bare Metal Admin Cluster's API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        /// Allows clients to perform consistent read-modify-writes
        /// through optimistic concurrency control.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Fleet related configuration.
        /// Fleets are a Google Cloud concept for logically organizing clusters,
        /// letting you use and manage multi-cluster capabilities and apply
        /// consistent policies across your systems.
        /// See [Anthos Fleets](https://cloud.google.com/anthos/multicluster-management/fleets) for
        /// more details on Anthos multi-cluster capabilities using Fleets.
        /// Structure is documented below.
        pub fleets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalAdminClusterFleet>,
        >,
        /// Specifies the load balancer configuration.
        /// Structure is documented below.
        pub load_balancer: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterLoadBalancer>,
        >,
        /// The object name of the Bare Metal Admin Cluster custom resource on the
        /// associated admin cluster. This field is used to support conflicting
        /// names when enrolling existing clusters to the API. When used as a part of
        /// cluster enrollment, this field will differ from the ID in the resource
        /// name. For new clusters, this field will match the user provided cluster ID
        /// and be visible in the last component of the resource name. It is not
        /// modifiable.
        /// All users should use this name to access their cluster using gkectl or
        /// kubectl and should expect to see the local name when viewing admin
        /// cluster controller logs.
        pub local_name: pulumi_gestalt_rust::Output<String>,
        /// The location of the resource.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the workload node configurations.
        /// Structure is documented below.
        pub maintenance_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::gkeonprem::BareMetalAdminClusterMaintenanceConfig,
            >,
        >,
        /// The bare metal admin cluster name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network configuration.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNetworkConfig>,
        >,
        /// Specifies the node access related settings for the bare metal user cluster.
        /// Structure is documented below.
        pub node_access_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNodeAccessConfig>,
        >,
        /// Specifies the workload node configurations.
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterNodeConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Specifies the cluster proxy configuration.
        /// Structure is documented below.
        pub proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterProxy>,
        >,
        /// If set, there are currently changes in flight to the Bare Metal Admin Cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the security related settings for the Bare Metal User Cluster.
        /// Structure is documented below.
        pub security_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterSecurityConfig>,
        >,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Specifies the detailed validation check status
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalAdminClusterStatus>,
        >,
        /// Specifies the cluster storage configuration.
        /// Structure is documented below.
        pub storage: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalAdminClusterStorage>,
        >,
        /// The unique identifier of the Bare Metal Admin Cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was last updated, in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the security related settings for the Bare Metal Admin Cluster.
        /// Structure is documented below.
        pub validation_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalAdminClusterValidationCheck>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BareMetalAdminClusterArgs,
    ) -> BareMetalAdminClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let bare_metal_version_binding_1 = args.bare_metal_version.get_output(context);
        let bare_metal_version_binding = bare_metal_version_binding_1.get_inner();
        let cluster_operations_binding_1 = args.cluster_operations.get_output(context);
        let cluster_operations_binding = cluster_operations_binding_1.get_inner();
        let control_plane_binding_1 = args.control_plane.get_output(context);
        let control_plane_binding = control_plane_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let load_balancer_binding_1 = args.load_balancer.get_output(context);
        let load_balancer_binding = load_balancer_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let maintenance_config_binding_1 = args.maintenance_config.get_output(context);
        let maintenance_config_binding = maintenance_config_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_config_binding_1 = args.network_config.get_output(context);
        let network_config_binding = network_config_binding_1.get_inner();
        let node_access_config_binding_1 = args.node_access_config.get_output(context);
        let node_access_config_binding = node_access_config_binding_1.get_inner();
        let node_config_binding_1 = args.node_config.get_output(context);
        let node_config_binding = node_config_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let proxy_binding_1 = args.proxy.get_output(context);
        let proxy_binding = proxy_binding_1.get_inner();
        let security_config_binding_1 = args.security_config.get_output(context);
        let security_config_binding = security_config_binding_1.get_inner();
        let storage_binding_1 = args.storage.get_output(context);
        let storage_binding = storage_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkeonprem/bareMetalAdminCluster:BareMetalAdminCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "bareMetalVersion".into(),
                    value: &bare_metal_version_binding,
                },
                register_interface::ObjectField {
                    name: "clusterOperations".into(),
                    value: &cluster_operations_binding,
                },
                register_interface::ObjectField {
                    name: "controlPlane".into(),
                    value: &control_plane_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfig".into(),
                    value: &maintenance_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "nodeAccessConfig".into(),
                    value: &node_access_config_binding,
                },
                register_interface::ObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "proxy".into(),
                    value: &proxy_binding,
                },
                register_interface::ObjectField {
                    name: "securityConfig".into(),
                    value: &security_config_binding,
                },
                register_interface::ObjectField {
                    name: "storage".into(),
                    value: &storage_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BareMetalAdminClusterResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            bare_metal_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bareMetalVersion"),
            ),
            cluster_operations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterOperations"),
            ),
            control_plane: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("controlPlane"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            fleets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fleets"),
            ),
            load_balancer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancer"),
            ),
            local_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maintenance_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfig"),
            ),
            node_access_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeAccessConfig"),
            ),
            node_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeConfig"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            proxy: pulumi_gestalt_rust::__private::into_domain(o.extract_field("proxy")),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            security_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityConfig"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storage"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            validation_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validationChecks"),
            ),
        }
    }
}
