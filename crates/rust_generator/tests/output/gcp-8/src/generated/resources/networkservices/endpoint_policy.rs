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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointPolicyArgs {
        /// This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic
        /// at the matched endpoints.
        #[builder(into, default)]
        pub authorization_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from
        /// the proxy to the actual endpoints.
        #[builder(into, default)]
        pub client_tls_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. A matcher that selects endpoints to which the policies should be applied.
        /// Structure is documented below.
        #[builder(into)]
        pub endpoint_matcher: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkservices::EndpointPolicyEndpointMatcher,
        >,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the EndpointPolicy resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be
        /// applied to terminate the inbound traffic at the identified backends.
        #[builder(into, default)]
        pub server_tls_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports.
        #[builder(into, default)]
        pub traffic_port_selector: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkservices::EndpointPolicyTrafficPortSelector,
            >,
        >,
        /// The type of endpoint policy. This is primarily used to validate the configuration.
        /// Possible values are: `SIDECAR_PROXY`, `GRPC_SERVER`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointPolicyResult {
        /// This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic
        /// at the matched endpoints.
        pub authorization_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from
        /// the proxy to the actual endpoints.
        pub client_tls_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the TcpRoute was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. A matcher that selects endpoints to which the policies should be applied.
        /// Structure is documented below.
        pub endpoint_matcher: pulumi_gestalt_rust::Output<
            super::super::types::networkservices::EndpointPolicyEndpointMatcher,
        >,
        /// Set of label tags associated with the TcpRoute resource. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the EndpointPolicy resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be
        /// applied to terminate the inbound traffic at the identified backends.
        pub server_tls_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports.
        pub traffic_port_selector: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkservices::EndpointPolicyTrafficPortSelector,
            >,
        >,
        /// The type of endpoint policy. This is primarily used to validate the configuration.
        /// Possible values are: `SIDECAR_PROXY`, `GRPC_SERVER`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Time the TcpRoute was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointPolicyArgs,
    ) -> EndpointPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorization_policy_binding = args.authorization_policy.get_output(context);
        let client_tls_policy_binding = args.client_tls_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let endpoint_matcher_binding = args.endpoint_matcher.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let server_tls_policy_binding = args.server_tls_policy.get_output(context);
        let traffic_port_selector_binding = args
            .traffic_port_selector
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/endpointPolicy:EndpointPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationPolicy".into(),
                    value: &authorization_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientTlsPolicy".into(),
                    value: &client_tls_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointMatcher".into(),
                    value: &endpoint_matcher_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverTlsPolicy".into(),
                    value: &server_tls_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficPortSelector".into(),
                    value: &traffic_port_selector_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointPolicyResult {
            authorization_policy: o.get_field("authorizationPolicy"),
            client_tls_policy: o.get_field("clientTlsPolicy"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            endpoint_matcher: o.get_field("endpointMatcher"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            server_tls_policy: o.get_field("serverTlsPolicy"),
            traffic_port_selector: o.get_field("trafficPortSelector"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
