/// Mange the named ports setting for a managed instance group without
/// managing the group as whole. This resource is primarily intended for use
/// with GKE-generated groups that shouldn't otherwise be managed by other
/// tools.
///
///
/// To get more information about InstanceGroupNamedPort, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroup)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/instance-groups/)
///
/// ## Example Usage
///
/// ### Instance Group Named Port Gke
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let containerNetwork = network::create(
///         "containerNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("container-network")
///             .build_struct(),
///     );
///     let containerSubnetwork = subnetwork::create(
///         "containerSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.36.0/24")
///             .name("container-subnetwork")
///             .network("${containerNetwork.name}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let myCluster = cluster::create(
///         "myCluster",
///         ClusterArgs::builder()
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .ip_allocation_policy(
///                 ClusterIpAllocationPolicy::builder()
///                     .clusterIpv4CidrBlock("/19")
///                     .servicesIpv4CidrBlock("/22")
///                     .build_struct(),
///             )
///             .location("us-central1-a")
///             .name("my-cluster")
///             .network("${containerNetwork.name}")
///             .subnetwork("${containerSubnetwork.name}")
///             .build_struct(),
///     );
///     let myPort = instance_group_named_port::create(
///         "myPort",
///         InstanceGroupNamedPortArgs::builder()
///             .group("${myCluster.nodePools[0].instanceGroupUrls[0]}")
///             .name("http")
///             .port(8080)
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let myPorts = instance_group_named_port::create(
///         "myPorts",
///         InstanceGroupNamedPortArgs::builder()
///             .group("${myCluster.nodePools[0].instanceGroupUrls[0]}")
///             .name("https")
///             .port(4443)
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// InstanceGroupNamedPort can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroups/{{group}}/{{port}}/{{name}}`
///
/// * `{{project}}/{{zone}}/{{group}}/{{port}}/{{name}}`
///
/// * `{{zone}}/{{group}}/{{port}}/{{name}}`
///
/// * `{{group}}/{{port}}/{{name}}`
///
/// When using the `pulumi import` command, InstanceGroupNamedPort can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupNamedPort:InstanceGroupNamedPort default projects/{{project}}/zones/{{zone}}/instanceGroups/{{group}}/{{port}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupNamedPort:InstanceGroupNamedPort default {{project}}/{{zone}}/{{group}}/{{port}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupNamedPort:InstanceGroupNamedPort default {{zone}}/{{group}}/{{port}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupNamedPort:InstanceGroupNamedPort default {{group}}/{{port}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_group_named_port {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupNamedPortArgs {
        /// The name of the instance group.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for this named port. The name must be 1-63 characters
        /// long, and comply with RFC1035.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number, which can be a value between 1 and 65535.
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance group.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupNamedPortResult {
        /// The name of the instance group.
        ///
        ///
        /// - - -
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The name for this named port. The name must be 1-63 characters
        /// long, and comply with RFC1035.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The port number, which can be a value between 1 and 65535.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The zone of the instance group.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupNamedPortArgs,
    ) -> InstanceGroupNamedPortResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let name_binding = args.name.get_output(context);
        let port_binding = args.port.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroupNamedPort:InstanceGroupNamedPort".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceGroupNamedPortResult {
            group: o.get_field("group"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            project: o.get_field("project"),
            zone: o.get_field("zone"),
        }
    }
}
