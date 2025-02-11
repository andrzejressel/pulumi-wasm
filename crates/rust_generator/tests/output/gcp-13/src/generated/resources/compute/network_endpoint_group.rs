/// Network endpoint groups (NEGs) are zonal resources that represent
/// collections of IP address and port combinations for GCP resources within a
/// single subnet. Each IP address and port combination is called a network
/// endpoint.
///
/// Network endpoint groups can be used as backends in backend services for
/// HTTP(S), TCP proxy, and SSL proxy load balancers. You cannot use NEGs as a
/// backend with internal load balancers. Because NEG backends allow you to
/// specify IP addresses and ports, you can distribute traffic in a granular
/// fashion among applications or containers running within VM instances.
///
/// Recreating a network endpoint group that's in use by another resource will give a
/// `resourceInUseByAnotherResource` error. Use `lifecycle.create_before_destroy`
/// to avoid this type of error.
///
///
/// To get more information about NetworkEndpointGroup, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/networkEndpointGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/)
///
/// ## Example Usage
///
/// ### Network Endpoint Group
///
///
/// ```yaml
/// resources:
///   neg:
///     type: gcp:compute:NetworkEndpointGroup
///     properties:
///       name: my-lb-neg
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       defaultPort: '90'
///       zone: us-central1-a
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: neg-network
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: neg-subnetwork
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${default.id}
/// ```
/// ### Network Endpoint Group Non Gcp
///
///
/// ```yaml
/// resources:
///   neg:
///     type: gcp:compute:NetworkEndpointGroup
///     properties:
///       name: my-lb-neg
///       network: ${default.id}
///       defaultPort: '90'
///       zone: us-central1-a
///       networkEndpointType: NON_GCP_PRIVATE_IP_PORT
///   default-endpoint:
///     type: gcp:compute:NetworkEndpoint
///     properties:
///       networkEndpointGroup: ${neg.name}
///       port: ${neg.defaultPort}
///       ipAddress: 127.0.0.1
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: neg-network
/// ```
///
/// ## Import
///
/// NetworkEndpointGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NetworkEndpointGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointGroup:NetworkEndpointGroup default projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointGroup:NetworkEndpointGroup default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointGroup:NetworkEndpointGroup default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointGroup:NetworkEndpointGroup default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkEndpointGroupArgs {
        /// The default port used if the port number is not specified in the
        /// network endpoint.
        #[builder(into, default)]
        pub default_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network to which all network endpoints in the NEG belong.
        /// Uses "default" project network if unspecified.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of network endpoints in this network endpoint group.
        /// NON_GCP_PRIVATE_IP_PORT is used for hybrid connectivity network
        /// endpoint groups (see https://cloud.google.com/load-balancing/docs/hybrid).
        /// Note that NON_GCP_PRIVATE_IP_PORT can only be used with Backend Services
        /// that 1) have the following load balancing schemes: EXTERNAL, EXTERNAL_MANAGED,
        /// INTERNAL_MANAGED, and INTERNAL_SELF_MANAGED and 2) support the RATE or
        /// CONNECTION balancing modes.
        /// Possible values include: GCE_VM_IP, GCE_VM_IP_PORT, NON_GCP_PRIVATE_IP_PORT, INTERNET_IP_PORT, INTERNET_FQDN_PORT, SERVERLESS, and PRIVATE_SERVICE_CONNECT.
        /// Default value is `GCE_VM_IP_PORT`.
        /// Possible values are: `GCE_VM_IP`, `GCE_VM_IP_PORT`, `NON_GCP_PRIVATE_IP_PORT`, `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`, `SERVERLESS`, `PRIVATE_SERVICE_CONNECT`.
        #[builder(into, default)]
        pub network_endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional subnetwork to which all network endpoints in the NEG belong.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone where the network endpoint group is located.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkEndpointGroupResult {
        /// The default port used if the port number is not specified in the
        /// network endpoint.
        pub default_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network to which all network endpoints in the NEG belong.
        /// Uses "default" project network if unspecified.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        /// NON_GCP_PRIVATE_IP_PORT is used for hybrid connectivity network
        /// endpoint groups (see https://cloud.google.com/load-balancing/docs/hybrid).
        /// Note that NON_GCP_PRIVATE_IP_PORT can only be used with Backend Services
        /// that 1) have the following load balancing schemes: EXTERNAL, EXTERNAL_MANAGED,
        /// INTERNAL_MANAGED, and INTERNAL_SELF_MANAGED and 2) support the RATE or
        /// CONNECTION balancing modes.
        /// Possible values include: GCE_VM_IP, GCE_VM_IP_PORT, NON_GCP_PRIVATE_IP_PORT, INTERNET_IP_PORT, INTERNET_FQDN_PORT, SERVERLESS, and PRIVATE_SERVICE_CONNECT.
        /// Default value is `GCE_VM_IP_PORT`.
        /// Possible values are: `GCE_VM_IP`, `GCE_VM_IP_PORT`, `NON_GCP_PRIVATE_IP_PORT`, `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`, `SERVERLESS`, `PRIVATE_SERVICE_CONNECT`.
        pub network_endpoint_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Number of network endpoints in the network endpoint group.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// Optional subnetwork to which all network endpoints in the NEG belong.
        pub subnetwork: pulumi_gestalt_rust::Output<Option<String>>,
        /// Zone where the network endpoint group is located.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkEndpointGroupArgs,
    ) -> NetworkEndpointGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_port_binding = args.default_port.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let network_endpoint_type_binding = args
            .network_endpoint_type
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkEndpointGroup:NetworkEndpointGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultPort".into(),
                    value: &default_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkEndpointType".into(),
                    value: &network_endpoint_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkEndpointGroupResult {
            default_port: o.get_field("defaultPort"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_endpoint_type: o.get_field("networkEndpointType"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            size: o.get_field("size"),
            subnetwork: o.get_field("subnetwork"),
            zone: o.get_field("zone"),
        }
    }
}
