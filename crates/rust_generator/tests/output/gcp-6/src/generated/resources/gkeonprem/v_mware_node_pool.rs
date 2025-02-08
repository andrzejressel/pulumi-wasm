/// A Google Vmware Node Pool.
///
///
///
/// ## Example Usage
///
/// ### Gkeonprem Vmware Node Pool Basic
///
///
/// ```yaml
/// resources:
///   default-basic:
///     type: gcp:gkeonprem:VMwareCluster
///     properties:
///       name: my-cluster
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       description: test cluster
///       onPremVersion: 1.13.1-gke.35
///       networkConfig:
///         serviceAddressCidrBlocks:
///           - 10.96.0.0/12
///         podAddressCidrBlocks:
///           - 192.168.0.0/16
///         dhcpIpConfig:
///           enabled: true
///       controlPlaneNode:
///         cpus: 4
///         memory: 8192
///         replicas: 1
///       loadBalancer:
///         vipConfig:
///           controlPlaneVip: 10.251.133.5
///           ingressVip: 10.251.135.19
///         metalLbConfig:
///           addressPools:
///             - pool: ingress-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///             - pool: lb-test-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///   nodepool-basic:
///     type: gcp:gkeonprem:VMwareNodePool
///     properties:
///       name: my-nodepool
///       location: us-west1
///       vmwareCluster: ${["default-basic"].name}
///       config:
///         replicas: 3
///         imageType: ubuntu_containerd
///         enableLoadBalancer: true
/// ```
/// ### Gkeonprem Vmware Node Pool Full
///
///
/// ```yaml
/// resources:
///   default-full:
///     type: gcp:gkeonprem:VMwareCluster
///     properties:
///       name: my-cluster
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       description: test cluster
///       onPremVersion: 1.13.1-gke.35
///       networkConfig:
///         serviceAddressCidrBlocks:
///           - 10.96.0.0/12
///         podAddressCidrBlocks:
///           - 192.168.0.0/16
///         dhcpIpConfig:
///           enabled: true
///       controlPlaneNode:
///         cpus: 4
///         memory: 8192
///         replicas: 1
///       loadBalancer:
///         vipConfig:
///           controlPlaneVip: 10.251.133.5
///           ingressVip: 10.251.135.19
///         metalLbConfig:
///           addressPools:
///             - pool: ingress-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///             - pool: lb-test-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///   nodepool-full:
///     type: gcp:gkeonprem:VMwareNodePool
///     properties:
///       name: my-nodepool
///       location: us-west1
///       vmwareCluster: ${["default-full"].name}
///       annotations: {}
///       config:
///         cpus: 4
///         memoryMb: 8196
///         replicas: 3
///         imageType: ubuntu_containerd
///         image: image
///         bootDiskSizeGb: 10
///         taints:
///           - key: key
///             value: value
///           - key: key
///             value: value
///             effect: NO_SCHEDULE
///         labels: {}
///         vsphereConfig:
///           datastore: test-datastore
///           tags:
///             - category: test-category-1
///               tag: tag-1
///             - category: test-category-2
///               tag: tag-2
///           hostGroups:
///             - host1
///             - host2
///         enableLoadBalancer: true
///       nodePoolAutoscaling:
///         minReplicas: 1
///         maxReplicas: 5
/// ```
///
/// ## Import
///
/// VmwareNodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vmwareClusters/{{vmware_cluster}}/vmwareNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{vmware_cluster}}/{{name}}`
///
/// * `{{location}}/{{vmware_cluster}}/{{name}}`
///
/// When using the `pulumi import` command, VmwareNodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareNodePool:VMwareNodePool default projects/{{project}}/locations/{{location}}/vmwareClusters/{{vmware_cluster}}/vmwareNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareNodePool:VMwareNodePool default {{project}}/{{location}}/{{vmware_cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareNodePool:VMwareNodePool default {{location}}/{{vmware_cluster}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_mware_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VMwareNodePoolArgs {
        /// Annotations on the node Pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys
        /// and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a
        /// slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with
        /// dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is non-authoritative, and will
        /// only manage the annotations present in your configuration. Please refer to the field 'effective_annotations' for all of
        /// the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The node configuration of the node pool.
        /// Structure is documented below.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::VMwareNodePoolConfig,
        >,
        /// The display name for the node pool.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The vmware node pool name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Node Pool autoscaling config for the node pool.
        #[builder(into, default)]
        pub node_pool_autoscaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareNodePoolNodePoolAutoscaling>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The cluster this node pool belongs to.
        #[builder(into)]
        pub vmware_cluster: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VMwareNodePoolResult {
        /// Annotations on the node Pool. This field has the same restrictions as Kubernetes annotations. The total size of all keys
        /// and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a
        /// slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with
        /// dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is non-authoritative, and will
        /// only manage the annotations present in your configuration. Please refer to the field 'effective_annotations' for all of
        /// the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The node configuration of the node pool.
        /// Structure is documented below.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareNodePoolConfig,
        >,
        /// The time the cluster was created, in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was deleted, in RFC3339 text format.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// The display name for the node pool.
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
        /// The vmware node pool name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Node Pool autoscaling config for the node pool.
        pub node_pool_autoscaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareNodePoolNodePoolAutoscaling>,
        >,
        /// Anthos version for the node pool. Defaults to the user cluster version.
        pub on_prem_version: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// If set, there are currently changes in flight to the node pool.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// ResourceStatus representing detailed cluster state.
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareNodePoolStatus>,
        >,
        /// The unique identifier of the node pool.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time the cluster was last updated, in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The cluster this node pool belongs to.
        pub vmware_cluster: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VMwareNodePoolArgs,
    ) -> VMwareNodePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let config_binding = args.config.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_pool_autoscaling_binding = args
            .node_pool_autoscaling
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let vmware_cluster_binding = args.vmware_cluster.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkeonprem/vMwareNodePool:VMwareNodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
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
                    name: "nodePoolAutoscaling".into(),
                    value: &node_pool_autoscaling_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vmwareCluster".into(),
                    value: &vmware_cluster_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VMwareNodePoolResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
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
            node_pool_autoscaling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePoolAutoscaling"),
            ),
            on_prem_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("onPremVersion"),
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
            vmware_cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareCluster"),
            ),
        }
    }
}
