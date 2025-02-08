/// An individual endpoint that provides a service.
///
/// To get more information about Endpoint, see:
///
/// * [API documentation](https://cloud.google.com/service-directory/docs/reference/rest/v1beta1/projects.locations.namespaces.services.endpoints)
/// * How-to Guides
///     * [Configuring an endpoint](https://cloud.google.com/service-directory/docs/configuring-service-directory#configuring_an_endpoint)
///
/// ## Example Usage
///
/// ### Service Directory Endpoint Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:servicedirectory:Namespace
///     properties:
///       namespaceId: example-namespace
///       location: us-central1
///   exampleService:
///     type: gcp:servicedirectory:Service
///     name: example
///     properties:
///       serviceId: example-service
///       namespace: ${example.id}
///   exampleEndpoint:
///     type: gcp:servicedirectory:Endpoint
///     name: example
///     properties:
///       endpointId: example-endpoint
///       service: ${exampleService.id}
///       metadata:
///         stage: prod
///         region: us-central1
///       address: 1.2.3.4
///       port: 5353
/// ```
/// ### Service Directory Endpoint With Network
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:compute:Network
///     properties:
///       name: example-network
///   exampleNamespace:
///     type: gcp:servicedirectory:Namespace
///     name: example
///     properties:
///       namespaceId: example-namespace
///       location: us-central1
///   exampleService:
///     type: gcp:servicedirectory:Service
///     name: example
///     properties:
///       serviceId: example-service
///       namespace: ${exampleNamespace.id}
///   exampleEndpoint:
///     type: gcp:servicedirectory:Endpoint
///     name: example
///     properties:
///       endpointId: example-endpoint
///       service: ${exampleService.id}
///       metadata:
///         stage: prod
///         region: us-central1
///       network: projects/${project.number}/locations/global/networks/${example.name}
///       address: 1.2.3.4
///       port: 5353
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Endpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}/services/{{service_id}}/endpoints/{{endpoint_id}}`
///
/// * `{{project}}/{{location}}/{{namespace_id}}/{{service_id}}/{{endpoint_id}}`
///
/// * `{{location}}/{{namespace_id}}/{{service_id}}/{{endpoint_id}}`
///
/// When using the `pulumi import` command, Endpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/endpoint:Endpoint default projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}/services/{{service_id}}/endpoints/{{endpoint_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/endpoint:Endpoint default {{project}}/{{location}}/{{namespace_id}}/{{service_id}}/{{endpoint_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/endpoint:Endpoint default {{location}}/{{namespace_id}}/{{service_id}}/{{endpoint_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// IPv4 or IPv6 address of the endpoint.
        #[builder(into, default)]
        pub address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Resource ID must be 1-63 characters long, including digits,
        /// lowercase letters or the hyphen character.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata for the endpoint. This data can be consumed
        /// by service clients. The entire metadata dictionary may contain
        /// up to 512 characters, spread across all key-value pairs.
        /// Metadata that goes beyond any these limits will be rejected.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URL to the network, such as projects/PROJECT_NUMBER/locations/global/networks/NETWORK_NAME.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Port that the endpoint is running on, must be in the
        /// range of [0, 65535]. If unspecified, the default is 0.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The resource name of the service that this endpoint provides.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// IPv4 or IPv6 address of the endpoint.
        pub address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Resource ID must be 1-63 characters long, including digits,
        /// lowercase letters or the hyphen character.
        ///
        ///
        /// - - -
        pub endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Metadata for the endpoint. This data can be consumed
        /// by service clients. The entire metadata dictionary may contain
        /// up to 512 characters, spread across all key-value pairs.
        /// Metadata that goes beyond any these limits will be rejected.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the endpoint in the format
        /// `projects/*/locations/*/namespaces/*/services/*/endpoints/*`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL to the network, such as projects/PROJECT_NUMBER/locations/global/networks/NETWORK_NAME.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Port that the endpoint is running on, must be in the
        /// range of [0, 65535]. If unspecified, the default is 0.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The resource name of the service that this endpoint provides.
        pub service: pulumi_gestalt_rust::Output<String>,
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
        let address_binding = args.address.get_output(context).get_inner();
        let endpoint_id_binding = args.endpoint_id.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:servicedirectory/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "address".into(),
                    value: &address_binding,
                },
                register_interface::ObjectField {
                    name: "endpointId".into(),
                    value: &endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointResult {
            address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("address"),
            ),
            endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointId"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
        }
    }
}
