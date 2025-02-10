/// A Google Bare Metal User Cluster.
///
///
///
/// ## Example Usage
///
/// ### Gkeonprem Bare Metal Cluster Basic
///
///
/// ```yaml
/// resources:
///   cluster-basic:
///     type: gcp:gkeonprem:BareMetalCluster
///     properties:
///       name: my-cluster
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       bareMetalVersion: 1.12.3
///       networkConfig:
///         islandModeCidr:
///           serviceAddressCidrBlocks:
///             - 172.26.0.0/16
///           podAddressCidrBlocks:
///             - 10.240.0.0/13
///       controlPlane:
///         controlPlaneNodePoolConfig:
///           nodePoolConfig:
///             labels: {}
///             operatingSystem: LINUX
///             nodeConfigs:
///               - labels: {}
///                 nodeIp: 10.200.0.9
///       loadBalancer:
///         portConfig:
///           controlPlaneLoadBalancerPort: 443
///         vipConfig:
///           controlPlaneVip: 10.200.0.13
///           ingressVip: 10.200.0.14
///         metalLbConfig:
///           addressPools:
///             - pool: pool1
///               addresses:
///                 - 10.200.0.14/32
///                 - 10.200.0.15/32
///                 - 10.200.0.16/32
///                 - 10.200.0.17/32
///                 - 10.200.0.18/32
///                 - fd00:1::f/128
///                 - fd00:1::10/128
///                 - fd00:1::11/128
///                 - fd00:1::12/128
///               avoidBuggyIps: true
///               manualAssign: true
///       storage:
///         lvpShareConfig:
///           lvpConfig:
///             path: /mnt/localpv-share
///             storageClass: local-shared
///           sharedPathPvCount: 5
///         lvpNodeMountsConfig:
///           path: /mnt/localpv-disk
///           storageClass: local-disks
///       securityConfig:
///         authorization:
///           adminUsers:
///             - username: admin@hashicorptest.com
/// ```
/// ### Gkeonprem Bare Metal Cluster Manuallb
///
///
/// ```yaml
/// resources:
///   cluster-manuallb:
///     type: gcp:gkeonprem:BareMetalCluster
///     properties:
///       name: cluster-manuallb
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       bareMetalVersion: 1.12.3
///       networkConfig:
///         islandModeCidr:
///           serviceAddressCidrBlocks:
///             - 172.26.0.0/16
///           podAddressCidrBlocks:
///             - 10.240.0.0/13
///       controlPlane:
///         controlPlaneNodePoolConfig:
///           nodePoolConfig:
///             labels: {}
///             operatingSystem: LINUX
///             nodeConfigs:
///               - labels: {}
///                 nodeIp: 10.200.0.9
///       loadBalancer:
///         portConfig:
///           controlPlaneLoadBalancerPort: 443
///         vipConfig:
///           controlPlaneVip: 10.200.0.13
///           ingressVip: 10.200.0.14
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
///       securityConfig:
///         authorization:
///           adminUsers:
///             - username: admin@hashicorptest.com
///       binaryAuthorization:
///         evaluationMode: DISABLED
///       upgradePolicy:
///         policy: SERIAL
/// ```
/// ### Gkeonprem Bare Metal Cluster Bgplb
///
///
/// ```yaml
/// resources:
///   cluster-bgplb:
///     type: gcp:gkeonprem:BareMetalCluster
///     properties:
///       name: cluster-bgplb
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       bareMetalVersion: 1.12.3
///       networkConfig:
///         islandModeCidr:
///           serviceAddressCidrBlocks:
///             - 172.26.0.0/16
///           podAddressCidrBlocks:
///             - 10.240.0.0/13
///         advancedNetworking: true
///         multipleNetworkInterfacesConfig:
///           enabled: true
///         srIovConfig:
///           enabled: true
///       controlPlane:
///         controlPlaneNodePoolConfig:
///           nodePoolConfig:
///             labels: {}
///             operatingSystem: LINUX
///             nodeConfigs:
///               - labels: {}
///                 nodeIp: 10.200.0.9
///             taints:
///               - key: test-key
///                 value: test-value
///                 effect: NO_EXECUTE
///         apiServerArgs:
///           - argument: test-argument
///             value: test-value
///       loadBalancer:
///         portConfig:
///           controlPlaneLoadBalancerPort: 443
///         vipConfig:
///           controlPlaneVip: 10.200.0.13
///           ingressVip: 10.200.0.14
///         bgpLbConfig:
///           asn: 123456
///           bgpPeerConfigs:
///             - asn: 123457
///               ipAddress: 10.0.0.1
///               controlPlaneNodes:
///                 - test-node
///           addressPools:
///             - pool: pool1
///               addresses:
///                 - 10.200.0.14/32
///                 - 10.200.0.15/32
///                 - 10.200.0.16/32
///                 - 10.200.0.17/32
///                 - 10.200.0.18/32
///                 - fd00:1::f/128
///                 - fd00:1::10/128
///                 - fd00:1::11/128
///                 - fd00:1::12/128
///           loadBalancerNodePoolConfig:
///             nodePoolConfig:
///               labels: {}
///               operatingSystem: LINUX
///               nodeConfigs:
///                 - labels: {}
///                   nodeIp: 10.200.0.9
///               taints:
///                 - key: test-key
///                   value: test-value
///                   effect: NO_EXECUTE
///               kubeletConfig:
///                 registryPullQps: 10
///                 registryBurst: 12
///                 serializeImagePullsDisabled: true
///       storage:
///         lvpShareConfig:
///           lvpConfig:
///             path: /mnt/localpv-share
///             storageClass: local-shared
///           sharedPathPvCount: 5
///         lvpNodeMountsConfig:
///           path: /mnt/localpv-disk
///           storageClass: local-disks
///       securityConfig:
///         authorization:
///           adminUsers:
///             - username: admin@hashicorptest.com
///       proxy:
///         uri: http://test-domain/test
///         noProxies:
///           - 127.0.0.1
///       clusterOperations:
///         enableApplicationLogs: true
///       maintenanceConfig:
///         maintenanceAddressCidrBlocks:
///           - 192.168.0.1/20
///       nodeConfig:
///         maxPodsPerNode: 10
///         containerRuntime: CONTAINERD
///       nodeAccessConfig:
///         loginUser: test@example.com
///       osEnvironmentConfig:
///         packageRepoExcluded: true
/// ```
///
/// ## Import
///
/// BareMetalCluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/bareMetalClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, BareMetalCluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalCluster:BareMetalCluster default projects/{{project}}/locations/{{location}}/bareMetalClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalCluster:BareMetalCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalCluster:BareMetalCluster default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bare_metal_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BareMetalClusterArgs {
        /// The Admin Cluster this Bare Metal User Cluster belongs to.
        /// This is the full resource name of the Admin Cluster's hub membership.
        #[builder(into)]
        pub admin_cluster_membership: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Annotations on the Bare Metal User Cluster. This field has the same restrictions as Kubernetes annotations. The total
        /// size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A human readable description of this Bare Metal User Cluster.
        #[builder(into)]
        pub bare_metal_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Binary Authorization related configurations.
        #[builder(into, default)]
        pub binary_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterBinaryAuthorization>,
        >,
        /// Specifies the User Cluster's observability infrastructure.
        #[builder(into, default)]
        pub cluster_operations: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterClusterOperations>,
        >,
        /// Specifies the control plane configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub control_plane: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::BareMetalClusterControlPlane,
        >,
        /// (Output)
        /// The description of the validation check.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the load balancer configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::BareMetalClusterLoadBalancer,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the workload node configurations.
        #[builder(into, default)]
        pub maintenance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterMaintenanceConfig>,
        >,
        /// The bare metal cluster name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::BareMetalClusterNetworkConfig,
        >,
        /// Specifies the node access related settings for the bare metal user cluster.
        #[builder(into, default)]
        pub node_access_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterNodeAccessConfig>,
        >,
        /// Specifies the workload node configurations.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterNodeConfig>,
        >,
        /// OS environment related configurations.
        #[builder(into, default)]
        pub os_environment_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterOsEnvironmentConfig>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the cluster proxy configuration.
        #[builder(into, default)]
        pub proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterProxy>,
        >,
        /// Specifies the security related settings for the Bare Metal User Cluster.
        #[builder(into, default)]
        pub security_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterSecurityConfig>,
        >,
        /// Specifies the cluster storage configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub storage: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::BareMetalClusterStorage,
        >,
        /// The cluster upgrade policy.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::BareMetalClusterUpgradePolicy>,
        >,
    }
    #[allow(dead_code)]
    pub struct BareMetalClusterResult {
        /// The Admin Cluster this Bare Metal User Cluster belongs to.
        /// This is the full resource name of the Admin Cluster's hub membership.
        pub admin_cluster_membership: pulumi_gestalt_rust::Output<String>,
        /// Annotations on the Bare Metal User Cluster. This field has the same restrictions as Kubernetes annotations. The total
        /// size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A human readable description of this Bare Metal User Cluster.
        pub bare_metal_version: pulumi_gestalt_rust::Output<String>,
        /// Binary Authorization related configurations.
        pub binary_authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterBinaryAuthorization>,
        >,
        /// Specifies the User Cluster's observability infrastructure.
        pub cluster_operations: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterClusterOperations>,
        >,
        /// Specifies the control plane configuration.
        /// Structure is documented below.
        pub control_plane: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::BareMetalClusterControlPlane,
        >,
        /// The time the cluster was created, in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was deleted, in RFC3339 text format.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// The description of the validation check.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP address name of Bare Metal User Cluster's API server.
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
            Vec<super::super::types::gkeonprem::BareMetalClusterFleet>,
        >,
        /// Specifies the load balancer configuration.
        /// Structure is documented below.
        pub load_balancer: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::BareMetalClusterLoadBalancer,
        >,
        /// The object name of the Bare Metal Cluster custom resource on the
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
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the workload node configurations.
        pub maintenance_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterMaintenanceConfig>,
        >,
        /// The bare metal cluster name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network configuration.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::BareMetalClusterNetworkConfig,
        >,
        /// Specifies the node access related settings for the bare metal user cluster.
        pub node_access_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterNodeAccessConfig>,
        >,
        /// Specifies the workload node configurations.
        pub node_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterNodeConfig>,
        >,
        /// OS environment related configurations.
        pub os_environment_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterOsEnvironmentConfig>,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Specifies the cluster proxy configuration.
        pub proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterProxy>,
        >,
        /// If set, there are currently changes in flight to the Bare Metal User Cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the security related settings for the Bare Metal User Cluster.
        pub security_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterSecurityConfig>,
        >,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Specifies the detailed validation check status
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalClusterStatus>,
        >,
        /// Specifies the cluster storage configuration.
        /// Structure is documented below.
        pub storage: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::BareMetalClusterStorage,
        >,
        /// The unique identifier of the Bare Metal User Cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was last updated, in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The cluster upgrade policy.
        pub upgrade_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::BareMetalClusterUpgradePolicy>,
        >,
        /// Specifies the security related settings for the Bare Metal User Cluster.
        /// Structure is documented below.
        pub validation_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalClusterValidationCheck>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BareMetalClusterArgs,
    ) -> BareMetalClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_cluster_membership_binding = args
            .admin_cluster_membership
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let bare_metal_version_binding = args.bare_metal_version.get_output(context);
        let binary_authorization_binding = args.binary_authorization.get_output(context);
        let cluster_operations_binding = args.cluster_operations.get_output(context);
        let control_plane_binding = args.control_plane.get_output(context);
        let description_binding = args.description.get_output(context);
        let load_balancer_binding = args.load_balancer.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_config_binding = args.maintenance_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let node_access_config_binding = args.node_access_config.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let os_environment_config_binding = args
            .os_environment_config
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_binding = args.proxy.get_output(context);
        let security_config_binding = args.security_config.get_output(context);
        let storage_binding = args.storage.get_output(context);
        let upgrade_policy_binding = args.upgrade_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkeonprem/bareMetalCluster:BareMetalCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminClusterMembership".into(),
                    value: admin_cluster_membership_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bareMetalVersion".into(),
                    value: bare_metal_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "binaryAuthorization".into(),
                    value: binary_authorization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterOperations".into(),
                    value: cluster_operations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlane".into(),
                    value: control_plane_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: load_balancer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfig".into(),
                    value: maintenance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeAccessConfig".into(),
                    value: node_access_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: node_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osEnvironmentConfig".into(),
                    value: os_environment_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxy".into(),
                    value: proxy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityConfig".into(),
                    value: security_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storage".into(),
                    value: storage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradePolicy".into(),
                    value: upgrade_policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BareMetalClusterResult {
            admin_cluster_membership: o.get_field("adminClusterMembership"),
            annotations: o.get_field("annotations"),
            bare_metal_version: o.get_field("bareMetalVersion"),
            binary_authorization: o.get_field("binaryAuthorization"),
            cluster_operations: o.get_field("clusterOperations"),
            control_plane: o.get_field("controlPlane"),
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            description: o.get_field("description"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            endpoint: o.get_field("endpoint"),
            etag: o.get_field("etag"),
            fleets: o.get_field("fleets"),
            load_balancer: o.get_field("loadBalancer"),
            local_name: o.get_field("localName"),
            location: o.get_field("location"),
            maintenance_config: o.get_field("maintenanceConfig"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            node_access_config: o.get_field("nodeAccessConfig"),
            node_config: o.get_field("nodeConfig"),
            os_environment_config: o.get_field("osEnvironmentConfig"),
            project: o.get_field("project"),
            proxy: o.get_field("proxy"),
            reconciling: o.get_field("reconciling"),
            security_config: o.get_field("securityConfig"),
            state: o.get_field("state"),
            statuses: o.get_field("statuses"),
            storage: o.get_field("storage"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            upgrade_policy: o.get_field("upgradePolicy"),
            validation_checks: o.get_field("validationChecks"),
        }
    }
}
