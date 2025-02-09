/// An endpoint indexes are deployed into. An index endpoint can have multiple deployed indexes.
///
///
/// To get more information about IndexEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints/)
///
/// ## Example Usage
///
/// ### Vertex Ai Index Endpoint
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
///     options:
///       dependsOn:
///         - ${vertexVpcConnection}
///   vertexVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vertex_vpc_connection
///     properties:
///       network: ${vertexNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${vertexRange.name}
///   vertexRange:
///     type: gcp:compute:GlobalAddress
///     name: vertex_range
///     properties:
///       name: address-name
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 24
///       network: ${vertexNetwork.id}
///   vertexNetwork:
///     type: gcp:compute:Network
///     name: vertex_network
///     properties:
///       name: network-name
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Index Endpoint With Psc
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       privateServiceConnectConfig:
///         enablePrivateServiceConnect: true
///         projectAllowlists:
///           - ${project.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Index Endpoint With Public Endpoint
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint with an public endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       publicEndpointEnabled: true
/// ```
///
/// ## Import
///
/// IndexEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/indexEndpoints/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, IndexEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default projects/{{project}}/locations/{{region}}/indexEndpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_index_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiIndexEndpointArgs {
        /// The description of the Index.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.
        /// Private services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.
        /// [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`.
        /// Where `{project}` is a project number, as in `12345`, and `{network}` is network name.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_service_connect_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiIndexEndpointPrivateServiceConnectConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, the deployed index will be accessible through public endpoint.
        #[builder(into, default)]
        pub public_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The region of the index endpoint. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiIndexEndpointResult {
        /// The timestamp of when the Index was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The description of the Index.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.
        /// Private services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.
        /// [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`.
        /// Where `{project}` is a project number, as in `12345`, and `{network}` is network name.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        pub private_service_connect_config: pulumi_gestalt_rust::Output<
            super::super::types::vertex::AiIndexEndpointPrivateServiceConnectConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// If publicEndpointEnabled is true, this field will be populated with the domain name to use for this index endpoint.
        pub public_endpoint_domain_name: pulumi_gestalt_rust::Output<String>,
        /// If true, the deployed index will be accessible through public endpoint.
        pub public_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the index endpoint. eg us-central1
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp of when the Index was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AiIndexEndpointArgs,
    ) -> AiIndexEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let network_binding_1 = args.network.get_output(context);
        let network_binding = network_binding_1.get_inner();
        let private_service_connect_config_binding_1 = args
            .private_service_connect_config
            .get_output(context);
        let private_service_connect_config_binding = private_service_connect_config_binding_1
            .get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let public_endpoint_enabled_binding_1 = args
            .public_endpoint_enabled
            .get_output(context);
        let public_endpoint_enabled_binding = public_endpoint_enabled_binding_1
            .get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiIndexEndpoint:AiIndexEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "privateServiceConnectConfig".into(),
                    value: &private_service_connect_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "publicEndpointEnabled".into(),
                    value: &public_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiIndexEndpointResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            private_service_connect_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateServiceConnectConfig"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            public_endpoint_domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicEndpointDomainName"),
            ),
            public_endpoint_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicEndpointEnabled"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
