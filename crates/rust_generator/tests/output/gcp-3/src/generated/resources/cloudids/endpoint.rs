/// Cloud IDS is an intrusion detection service that provides threat detection for intrusions, malware, spyware, and command-and-control attacks on your network.
///
///
/// To get more information about Endpoint, see:
///
/// * [API documentation](https://cloud.google.com/intrusion-detection-system/docs/configuring-ids)
///
/// ## Example Usage
///
/// ### Cloudids Endpoint
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: tf-test-my-network
///   serviceRange:
///     type: gcp:compute:GlobalAddress
///     name: service_range
///     properties:
///       name: address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${default.id}
///   privateServiceConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_service_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${serviceRange.name}
///   example-endpoint:
///     type: gcp:cloudids:Endpoint
///     properties:
///       name: test
///       location: us-central1-f
///       network: ${default.id}
///       severity: INFORMATIONAL
///     options:
///       dependsOn:
///         - ${privateServiceConnection}
/// ```
///
/// ## Import
///
/// Endpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/endpoints/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Endpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudids/endpoint:Endpoint default projects/{{project}}/locations/{{location}}/endpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudids/endpoint:Endpoint default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudids/endpoint:Endpoint default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// An optional description of the endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the endpoint.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the endpoint in the format projects/{project_id}/locations/{locationId}/endpoints/{endpointId}.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the VPC network that is connected to the IDS endpoint. This can either contain the VPC network name itself (like "src-net") or the full URL to the network (like "projects/{project_id}/global/networks/src-net").
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum alert severity level that is reported by the endpoint.
        /// Possible values are: `INFORMATIONAL`, `LOW`, `MEDIUM`, `HIGH`, `CRITICAL`.
        #[builder(into)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for threat IDs excluded from generating alerts. Limit: 99 IDs.
        #[builder(into, default)]
        pub threat_exceptions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// Creation timestamp in RFC 3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// URL of the endpoint's network address to which traffic is to be sent by Packet Mirroring.
        pub endpoint_forwarding_rule: pulumi_gestalt_rust::Output<String>,
        /// Internal IP address of the endpoint's network entry point.
        pub endpoint_ip: pulumi_gestalt_rust::Output<String>,
        /// The location for the endpoint.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the endpoint in the format projects/{project_id}/locations/{locationId}/endpoints/{endpointId}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the VPC network that is connected to the IDS endpoint. This can either contain the VPC network name itself (like "src-net") or the full URL to the network (like "projects/{project_id}/global/networks/src-net").
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The minimum alert severity level that is reported by the endpoint.
        /// Possible values are: `INFORMATIONAL`, `LOW`, `MEDIUM`, `HIGH`, `CRITICAL`.
        pub severity: pulumi_gestalt_rust::Output<String>,
        /// Configuration for threat IDs excluded from generating alerts. Limit: 99 IDs.
        pub threat_exceptions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Last update timestamp in RFC 3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let severity_binding = args.severity.get_output(context).get_inner();
        let threat_exceptions_binding = args
            .threat_exceptions
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudids/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "threatExceptions".into(),
                    value: &threat_exceptions_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            endpoint_forwarding_rule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointForwardingRule"),
            ),
            endpoint_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointIp"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            severity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            threat_exceptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatExceptions"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
