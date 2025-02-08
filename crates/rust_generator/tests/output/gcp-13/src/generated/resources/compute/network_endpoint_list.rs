/// A set of network endpoints belonging to a network endpoint group (NEG). A
/// single network endpoint represents a IP address and port combination that is
/// part of a specific network endpoint group  (NEG). NEGs are zonal collections
/// of these endpoints for GCP resources within a single subnet. **NOTE**:
/// Network endpoints cannot be created outside of a network endpoint group.
///
/// This resource is authoritative for a single NEG. Any endpoints not specified
/// by this resource will be deleted when the resource configuration is applied.
///
/// > **NOTE** In case the Endpoint's Instance is recreated, it's needed to
/// perform `apply` twice. To avoid situations like this, please use this resource
/// with the lifecycle `replace_triggered_by` method, with the passed Instance's ID.
///
///
/// To get more information about NetworkEndpoints, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/networkEndpointGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/)
///
/// ## Example Usage
///
/// ### Network Endpoints
///
///
/// ```yaml
/// resources:
///   default-endpoints:
///     type: gcp:compute:NetworkEndpointList
///     properties:
///       networkEndpointGroup: ${neg.name}
///       networkEndpoints:
///         - instance: ${["endpoint-instance1"].name}
///           port: ${neg.defaultPort}
///           ipAddress: ${["endpoint-instance1"].networkInterfaces[0].networkIp}
///         - instance: ${["endpoint-instance2"].name}
///           port: ${neg.defaultPort}
///           ipAddress: ${["endpoint-instance2"].networkInterfaces[0].networkIp}
///   endpoint-instance1:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           subnetwork: ${defaultSubnetwork.id}
///       name: endpoint-instance1
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: ${myImage.selfLink}
///   endpoint-instance2:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           subnetwork: ${defaultSubnetwork.id}
///       name: endpoint-instance2
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
/// NetworkEndpoints can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{network_endpoint_group}}`
///
/// * `{{project}}/{{zone}}/{{network_endpoint_group}}`
///
/// * `{{zone}}/{{network_endpoint_group}}`
///
/// * `{{network_endpoint_group}}`
///
/// When using the `pulumi import` command, NetworkEndpoints can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointList:NetworkEndpointList default projects/{{project}}/zones/{{zone}}/networkEndpointGroups/{{network_endpoint_group}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointList:NetworkEndpointList default {{project}}/{{zone}}/{{network_endpoint_group}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointList:NetworkEndpointList default {{zone}}/{{network_endpoint_group}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkEndpointList:NetworkEndpointList default {{network_endpoint_group}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_endpoint_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkEndpointListArgs {
        /// The network endpoint group these endpoints are part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network_endpoint_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The network endpoints to be added to the enclosing network endpoint group
        /// (NEG). Each endpoint specifies an IP address and port, along with
        /// additional information depending on the NEG type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::NetworkEndpointListNetworkEndpoint>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone where the containing network endpoint group is located.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkEndpointListResult {
        /// The network endpoint group these endpoints are part of.
        ///
        ///
        /// - - -
        pub network_endpoint_group: pulumi_gestalt_rust::Output<String>,
        /// The network endpoints to be added to the enclosing network endpoint group
        /// (NEG). Each endpoint specifies an IP address and port, along with
        /// additional information depending on the NEG type.
        /// Structure is documented below.
        pub network_endpoints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::NetworkEndpointListNetworkEndpoint>>,
        >,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkEndpointListArgs,
    ) -> NetworkEndpointListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let network_endpoint_group_binding = args
            .network_endpoint_group
            .get_output(context)
            .get_inner();
        let network_endpoints_binding = args
            .network_endpoints
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/networkEndpointList:NetworkEndpointList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkEndpointGroup".into(),
                    value: &network_endpoint_group_binding,
                },
                register_interface::ObjectField {
                    name: "networkEndpoints".into(),
                    value: &network_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkEndpointListResult {
            network_endpoint_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkEndpointGroup"),
            ),
            network_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkEndpoints"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
