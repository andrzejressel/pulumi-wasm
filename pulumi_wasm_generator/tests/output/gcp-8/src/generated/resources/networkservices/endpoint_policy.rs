/// ## Example Usage
///
/// ### Network Services Endpoint Policy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:EndpointPolicy
///     properties:
///       name: my-endpoint-policy
///       labels:
///         foo: bar
///       description: my description
///       type: SIDECAR_PROXY
///       trafficPortSelector:
///         ports:
///           - '8081'
///       endpointMatcher:
///         metadataLabelMatcher:
///           metadataLabelMatchCriteria: MATCH_ANY
///           metadataLabels:
///             - labelName: foo
///               labelValue: bar
/// ```
/// ### Network Services Endpoint Policy Empty Match
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:EndpointPolicy
///     properties:
///       name: my-endpoint-policy
///       labels:
///         foo: bar
///       description: my description
///       type: SIDECAR_PROXY
///       trafficPortSelector:
///         ports:
///           - '8081'
///       endpointMatcher:
///         metadataLabelMatcher:
///           metadataLabelMatchCriteria: MATCH_ANY
/// ```
///
/// ## Import
///
/// EndpointPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/endpointPolicies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EndpointPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/endpointPolicy:EndpointPolicy default projects/{{project}}/locations/global/endpointPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/endpointPolicy:EndpointPolicy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/endpointPolicy:EndpointPolicy default {{name}}
/// ```
///
pub mod endpoint_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointPolicyArgs {
        /// This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic
        /// at the matched endpoints.
        #[builder(into, default)]
        pub authorization_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from
        /// the proxy to the actual endpoints.
        #[builder(into, default)]
        pub client_tls_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. A matcher that selects endpoints to which the policies should be applied.
        /// Structure is documented below.
        #[builder(into)]
        pub endpoint_matcher: pulumi_wasm_rust::Output<
            super::super::types::networkservices::EndpointPolicyEndpointMatcher,
        >,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the EndpointPolicy resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be
        /// applied to terminate the inbound traffic at the identified backends.
        #[builder(into, default)]
        pub server_tls_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports.
        #[builder(into, default)]
        pub traffic_port_selector: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EndpointPolicyTrafficPortSelector,
            >,
        >,
        /// The type of endpoint policy. This is primarily used to validate the configuration.
        /// Possible values are: `SIDECAR_PROXY`, `GRPC_SERVER`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointPolicyResult {
        /// This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic
        /// at the matched endpoints.
        pub authorization_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from
        /// the proxy to the actual endpoints.
        pub client_tls_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Time the TcpRoute was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. A matcher that selects endpoints to which the policies should be applied.
        /// Structure is documented below.
        pub endpoint_matcher: pulumi_wasm_rust::Output<
            super::super::types::networkservices::EndpointPolicyEndpointMatcher,
        >,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the EndpointPolicy resource.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be
        /// applied to terminate the inbound traffic at the identified backends.
        pub server_tls_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports.
        pub traffic_port_selector: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EndpointPolicyTrafficPortSelector,
            >,
        >,
        /// The type of endpoint policy. This is primarily used to validate the configuration.
        /// Possible values are: `SIDECAR_PROXY`, `GRPC_SERVER`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Time the TcpRoute was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointPolicyArgs) -> EndpointPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorization_policy_binding = args.authorization_policy.get_inner();
        let client_tls_policy_binding = args.client_tls_policy.get_inner();
        let description_binding = args.description.get_inner();
        let endpoint_matcher_binding = args.endpoint_matcher.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let server_tls_policy_binding = args.server_tls_policy.get_inner();
        let traffic_port_selector_binding = args.traffic_port_selector.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/endpointPolicy:EndpointPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizationPolicy".into(),
                    value: &authorization_policy_binding,
                },
                register_interface::ObjectField {
                    name: "clientTlsPolicy".into(),
                    value: &client_tls_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "endpointMatcher".into(),
                    value: &endpoint_matcher_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serverTlsPolicy".into(),
                    value: &server_tls_policy_binding,
                },
                register_interface::ObjectField {
                    name: "trafficPortSelector".into(),
                    value: &traffic_port_selector_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationPolicy".into(),
                },
                register_interface::ResultField {
                    name: "clientTlsPolicy".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "endpointMatcher".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "serverTlsPolicy".into(),
                },
                register_interface::ResultField {
                    name: "trafficPortSelector".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointPolicyResult {
            authorization_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationPolicy").unwrap(),
            ),
            client_tls_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientTlsPolicy").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            endpoint_matcher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointMatcher").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            server_tls_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverTlsPolicy").unwrap(),
            ),
            traffic_port_selector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficPortSelector").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
