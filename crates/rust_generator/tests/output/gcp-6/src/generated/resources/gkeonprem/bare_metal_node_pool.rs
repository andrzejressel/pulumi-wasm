/// A Google Bare Metal Node Pool.
///
///
///
/// ## Example Usage
///
/// ### Gkeonprem Bare Metal Node Pool Basic
///
///
/// ```yaml
/// resources:
///   default-basic:
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
///   nodepool-basic:
///     type: gcp:gkeonprem:BareMetalNodePool
///     properties:
///       name: my-nodepool
///       bareMetalCluster: ${["default-basic"].name}
///       location: us-west1
///       nodePoolConfig:
///         operatingSystem: LINUX
///         nodeConfigs:
///           - nodeIp: 10.200.0.11
/// ```
/// ### Gkeonprem Bare Metal Node Pool Full
///
///
/// ```yaml
/// resources:
///   default-full:
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
///   nodepool-full:
///     type: gcp:gkeonprem:BareMetalNodePool
///     properties:
///       name: my-nodepool
///       displayName: test-name
///       bareMetalCluster: ${["default-full"].name}
///       location: us-west1
///       annotations: {}
///       nodePoolConfig:
///         operatingSystem: LINUX
///         labels: {}
///         nodeConfigs:
///           - nodeIp: 10.200.0.11
///             labels: {}
///         taints:
///           - key: test-key
///             value: test-value
///             effect: NO_EXECUTE
/// ```
///
/// ## Import
///
/// BareMetalNodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/bareMetalClusters/{{bare_metal_cluster}}/bareMetalNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{bare_metal_cluster}}/{{name}}`
///
/// * `{{location}}/{{bare_metal_cluster}}/{{name}}`
///
/// When using the `pulumi import` command, BareMetalNodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalNodePool:BareMetalNodePool default projects/{{project}}/locations/{{location}}/bareMetalClusters/{{bare_metal_cluster}}/bareMetalNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalNodePool:BareMetalNodePool default {{project}}/{{location}}/{{bare_metal_cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/bareMetalNodePool:BareMetalNodePool default {{location}}/{{bare_metal_cluster}}/{{name}}
/// ```
///
pub mod bare_metal_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BareMetalNodePoolArgs {
        /// Annotations on the Bare Metal Node Pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The cluster this node pool belongs to.
        #[builder(into)]
        pub bare_metal_cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name for the Bare Metal Node Pool.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The bare metal node pool name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Node pool configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub node_pool_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::BareMetalNodePoolNodePoolConfig,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BareMetalNodePoolResult {
        /// Annotations on the Bare Metal Node Pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The cluster this node pool belongs to.
        pub bare_metal_cluster: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was created, in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was deleted, in RFC3339 text format.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// The display name for the Bare Metal Node Pool.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        /// Allows clients to perform consistent read-modify-writes
        /// through optimistic concurrency control.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The bare metal node pool name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Node pool configuration.
        /// Structure is documented below.
        pub node_pool_config: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::BareMetalNodePoolNodePoolConfig,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// If set, there are currently changes in flight to the Bare Metal User Cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Specifies detailed node pool status.
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::BareMetalNodePoolStatus>,
        >,
        /// The unique identifier of the Bare Metal Node Pool.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was last updated, in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BareMetalNodePoolArgs,
    ) -> BareMetalNodePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let bare_metal_cluster_binding = args
            .bare_metal_cluster
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_pool_config_binding = args
            .node_pool_config
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkeonprem/bareMetalNodePool:BareMetalNodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "bareMetalCluster".into(),
                    value: &bare_metal_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodePoolConfig".into(),
                    value: &node_pool_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BareMetalNodePoolResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            bare_metal_cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bareMetalCluster"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            node_pool_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePoolConfig"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
