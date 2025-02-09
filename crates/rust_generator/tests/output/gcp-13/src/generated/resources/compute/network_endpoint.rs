/// A Network endpoint represents a IP address and port combination that is
/// part of a specific network endpoint group (NEG). NEGs are zonal
/// collections of these endpoints for GCP resources within a
/// single subnet. **NOTE**: Network endpoints cannot be created outside of a
/// network endpoint group.
///
/// > **NOTE** In case the Endpoint's Instance is recreated, it's needed to
/// perform `apply` twice. To avoid situations like this, please use this resource
/// with the lifecycle `replace_triggered_by` method, with the passed Instance's ID.
///
///
/// To get more information about NetworkEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/networkEndpointGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/)
///
/// ## Example Usage
///
/// ### Network Endpoint
///
///
/// ```yaml
/// resources:
///   default-endpoint:
///     type: gcp:compute:NetworkEndpoint
///     properties:
///       networkEndpointGroup: ${neg.name}
///       instance: ${["endpoint-instance"].name}
///       port: ${neg.defaultPort}
///       ipAddress: ${["endpoint-instance"].networkInterfaces[0].networkIp}
///   endpoint-instance:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           subnetwork: ${defaultSubnetwork.id}
///       name: endpoint-instance
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: ${myImage.selfLink}
///   group:
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
///       ipCidrRange: 10.0.0.1/16
///       region: us-central1
///       network: ${default.id}
/// variables:
///   myImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// NetworkEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}`
///
/// * `{{project}}/{{zone}}/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}`
///
/// * `{{zone}}/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}`
///
/// * `{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}`
///
/// When using the `pulumi import` command, NetworkEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpoint:NetworkEndpoint default projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpoint:NetworkEndpoint default {{project}}/{{zone}}/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpoint:NetworkEndpoint default {{zone}}/{{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpoint:NetworkEndpoint default {{network_endpoint_group}}/{{instance}}/{{ip_address}}/{{port}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkEndpointArgs {
        /// The name for a specific VM instance that the IP address belongs to.
        /// This is required for network endpoints of type GCE_VM_IP_PORT.
        /// The instance must be in the same zone of network endpoint group.
        #[builder(into, default)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IPv4 address of network endpoint. The IP address must belong
        /// to a VM in GCE (either the primary IP or as part of an aliased IP
        /// range).
        #[builder(into)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network_endpoint_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Port number of network endpoint.
        /// **Note** `port` is required unless the Network Endpoint Group is created
        /// with the type of `GCE_VM_IP`
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone where the containing network endpoint group is located.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkEndpointResult {
        /// The name for a specific VM instance that the IP address belongs to.
        /// This is required for network endpoints of type GCE_VM_IP_PORT.
        /// The instance must be in the same zone of network endpoint group.
        pub instance: pulumi_gestalt_rust::Output<Option<String>>,
        /// IPv4 address of network endpoint. The IP address must belong
        /// to a VM in GCE (either the primary IP or as part of an aliased IP
        /// range).
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        pub network_endpoint_group: pulumi_gestalt_rust::Output<String>,
        /// Port number of network endpoint.
        /// **Note** `port` is required unless the Network Endpoint Group is created
        /// with the type of `GCE_VM_IP`
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Zone where the containing network endpoint group is located.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkEndpointArgs,
    ) -> NetworkEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_binding = args.instance.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let network_endpoint_group_binding = args
            .network_endpoint_group
            .get_output(context);
        let port_binding = args.port.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkEndpoint:NetworkEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkEndpointGroup".into(),
                    value: network_endpoint_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkEndpointResult {
            instance: o.get_field("instance"),
            ip_address: o.get_field("ipAddress"),
            network_endpoint_group: o.get_field("networkEndpointGroup"),
            port: o.get_field("port"),
            project: o.get_field("project"),
            zone: o.get_field("zone"),
        }
    }
}
