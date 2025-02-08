/// Gateway represents the configuration for a proxy, typically a load balancer.
/// It captures the ip:port over which the services are exposed by the proxy,
/// along with any policy configurations. Routes have reference to to Gateways
/// to dictate how requests should be routed by this Gateway.
///
///
/// To get more information about Gateway, see:
///
/// * [API documentation](https://cloud.google.com/traffic-director/docs/reference/network-services/rest/v1/projects.locations.gateways)
///
/// ## Example Usage
///
/// ### Network Services Gateway Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = gateway::create(
///         "default",
///         GatewayArgs::builder()
///             .name("my-gateway")
///             .ports(vec![443,])
///             .scope("default-scope-basic")
///             .type_("OPEN_MESH")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Gateway Advanced
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:Gateway
///     properties:
///       name: my-gateway
///       labels:
///         foo: bar
///       description: my description
///       type: OPEN_MESH
///       ports:
///         - 443
///       scope: default-scope-advance
/// ```
/// ### Network Services Gateway Secure Web Proxy
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:Certificate
///     properties:
///       name: my-certificate
///       location: us-central1
///       selfManaged:
///         pemCertificate:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/cert.pem
///             return: result
///         pemPrivateKey:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/private-key.pem
///             return: result
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: my-network
///       routingMode: REGIONAL
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: my-subnetwork-name
///       purpose: PRIVATE
///       ipCidrRange: 10.128.0.0/20
///       region: us-central1
///       network: ${defaultNetwork.id}
///       role: ACTIVE
///   proxyonlysubnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-proxy-only-subnetwork
///       purpose: REGIONAL_MANAGED_PROXY
///       ipCidrRange: 192.168.0.0/23
///       region: us-central1
///       network: ${defaultNetwork.id}
///       role: ACTIVE
///   defaultGatewaySecurityPolicy:
///     type: gcp:networksecurity:GatewaySecurityPolicy
///     name: default
///     properties:
///       name: my-policy-name
///       location: us-central1
///   defaultGatewaySecurityPolicyRule:
///     type: gcp:networksecurity:GatewaySecurityPolicyRule
///     name: default
///     properties:
///       name: my-policyrule-name
///       location: us-central1
///       gatewaySecurityPolicy: ${defaultGatewaySecurityPolicy.name}
///       enabled: true
///       priority: 1
///       sessionMatcher: host() == 'example.com'
///       basicProfile: ALLOW
///   defaultGateway:
///     type: gcp:networkservices:Gateway
///     name: default
///     properties:
///       name: my-gateway1
///       location: us-central1
///       addresses:
///         - 10.128.0.99
///       type: SECURE_WEB_GATEWAY
///       ports:
///         - 443
///       scope: my-default-scope1
///       certificateUrls:
///         - ${default.id}
///       gatewaySecurityPolicy: ${defaultGatewaySecurityPolicy.id}
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       deleteSwgAutogenRouterOnDestroy: true
///     options:
///       dependsOn:
///         - ${proxyonlysubnet}
/// ```
/// ### Network Services Gateway Multiple Swp Same Network
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:Certificate
///     properties:
///       name: my-certificate
///       location: us-south1
///       selfManaged:
///         pemCertificate:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/cert.pem
///             return: result
///         pemPrivateKey:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/private-key.pem
///             return: result
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: my-network
///       routingMode: REGIONAL
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: my-subnetwork-name
///       purpose: PRIVATE
///       ipCidrRange: 10.128.0.0/20
///       region: us-south1
///       network: ${defaultNetwork.id}
///       role: ACTIVE
///   proxyonlysubnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-proxy-only-subnetwork
///       purpose: REGIONAL_MANAGED_PROXY
///       ipCidrRange: 192.168.0.0/23
///       region: us-south1
///       network: ${defaultNetwork.id}
///       role: ACTIVE
///   defaultGatewaySecurityPolicy:
///     type: gcp:networksecurity:GatewaySecurityPolicy
///     name: default
///     properties:
///       name: my-policy-name
///       location: us-south1
///   defaultGatewaySecurityPolicyRule:
///     type: gcp:networksecurity:GatewaySecurityPolicyRule
///     name: default
///     properties:
///       name: my-policyrule-name
///       location: us-south1
///       gatewaySecurityPolicy: ${defaultGatewaySecurityPolicy.name}
///       enabled: true
///       priority: 1
///       sessionMatcher: host() == 'example.com'
///       basicProfile: ALLOW
///   defaultGateway:
///     type: gcp:networkservices:Gateway
///     name: default
///     properties:
///       name: my-gateway1
///       location: us-south1
///       addresses:
///         - 10.128.0.99
///       type: SECURE_WEB_GATEWAY
///       ports:
///         - 443
///       scope: my-default-scope1
///       certificateUrls:
///         - ${default.id}
///       gatewaySecurityPolicy: ${defaultGatewaySecurityPolicy.id}
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       deleteSwgAutogenRouterOnDestroy: true
///     options:
///       dependsOn:
///         - ${proxyonlysubnet}
///   gateway2:
///     type: gcp:networkservices:Gateway
///     properties:
///       name: my-gateway2
///       location: us-south1
///       addresses:
///         - 10.128.0.98
///       type: SECURE_WEB_GATEWAY
///       ports:
///         - 443
///       scope: my-default-scope2
///       certificateUrls:
///         - ${default.id}
///       gatewaySecurityPolicy: ${defaultGatewaySecurityPolicy.id}
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       deleteSwgAutogenRouterOnDestroy: true
///     options:
///       dependsOn:
///         - ${proxyonlysubnet}
/// ```
///
/// ## Import
///
/// Gateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/gateways/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Gateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/gateway:Gateway default projects/{{project}}/locations/{{location}}/gateways/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/gateway:Gateway default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/gateway:Gateway default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayArgs {
        /// Zero or one IPv4-address on which the Gateway will receive the traffic. When no address is provided,
        /// an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'.
        /// Gateways of type 'OPEN_MESH' listen on 0.0.0.0.
        #[builder(into, default)]
        pub addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.
        /// This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'.
        #[builder(into, default)]
        pub certificate_urls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// When deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.
        /// If there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted.
        #[builder(into, default)]
        pub delete_swg_autogen_router_on_destroy: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.
        /// For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`.
        /// This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'.
        #[builder(into, default)]
        pub gateway_security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the Gateway resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short name of the Gateway resource to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The relative resource name identifying the VPC network that is using this configuration.
        /// For example: `projects/*/global/networks/network-1`.
        /// Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more port numbers (1-65535), on which the Gateway will receive traffic.
        /// The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are
        /// limited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports.
        #[builder(into)]
        pub ports: pulumi_gestalt_rust::InputOrOutput<Vec<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY.
        /// Possible values are: `NEXT_HOP_ROUTING_MODE`.
        #[builder(into, default)]
        pub routing_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. Scope determines how configuration across multiple Gateway instances are merged.
        /// The configuration for multiple Gateway instances with the same scope will be merged as presented as
        /// a single coniguration to the proxy/load balancer.
        /// Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated.
        /// If empty, TLS termination is disabled.
        #[builder(into, default)]
        pub server_tls_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The relative resource name identifying the subnetwork in which this SWG is allocated.
        /// For example: `projects/*/regions/us-central1/subnetworks/network-1`.
        /// Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. The type of the customer-managed gateway. Possible values are: * OPEN_MESH * SECURE_WEB_GATEWAY.
        /// Possible values are: `TYPE_UNSPECIFIED`, `OPEN_MESH`, `SECURE_WEB_GATEWAY`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GatewayResult {
        /// Zero or one IPv4-address on which the Gateway will receive the traffic. When no address is provided,
        /// an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'.
        /// Gateways of type 'OPEN_MESH' listen on 0.0.0.0.
        pub addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection.
        /// This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'.
        pub certificate_urls: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Time the AccessPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// When deleting a gateway of type 'SECURE_WEB_GATEWAY', this boolean option will also delete auto generated router by the gateway creation.
        /// If there is no other gateway of type 'SECURE_WEB_GATEWAY' remaining for that region and network it will be deleted.
        pub delete_swg_autogen_router_on_destroy: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections.
        /// For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`.
        /// This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'.
        pub gateway_security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of label tags associated with the Gateway resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Short name of the Gateway resource to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The relative resource name identifying the VPC network that is using this configuration.
        /// For example: `projects/*/global/networks/network-1`.
        /// Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more port numbers (1-65535), on which the Gateway will receive traffic.
        /// The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are
        /// limited to 1 port. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 and support multiple ports.
        pub ports: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY.
        /// Possible values are: `NEXT_HOP_ROUTING_MODE`.
        pub routing_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Immutable. Scope determines how configuration across multiple Gateway instances are merged.
        /// The configuration for multiple Gateway instances with the same scope will be merged as presented as
        /// a single coniguration to the proxy/load balancer.
        /// Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens.
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated.
        /// If empty, TLS termination is disabled.
        pub server_tls_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The relative resource name identifying the subnetwork in which this SWG is allocated.
        /// For example: `projects/*/regions/us-central1/subnetworks/network-1`.
        /// Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY.
        pub subnetwork: pulumi_gestalt_rust::Output<Option<String>>,
        /// Immutable. The type of the customer-managed gateway. Possible values are: * OPEN_MESH * SECURE_WEB_GATEWAY.
        /// Possible values are: `TYPE_UNSPECIFIED`, `OPEN_MESH`, `SECURE_WEB_GATEWAY`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Time the AccessPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GatewayArgs,
    ) -> GatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let addresses_binding = args.addresses.get_output(context).get_inner();
        let certificate_urls_binding = args
            .certificate_urls
            .get_output(context)
            .get_inner();
        let delete_swg_autogen_router_on_destroy_binding = args
            .delete_swg_autogen_router_on_destroy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let gateway_security_policy_binding = args
            .gateway_security_policy
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let ports_binding = args.ports.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let routing_mode_binding = args.routing_mode.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let server_tls_policy_binding = args
            .server_tls_policy
            .get_output(context)
            .get_inner();
        let subnetwork_binding = args.subnetwork.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/gateway:Gateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addresses".into(),
                    value: &addresses_binding,
                },
                register_interface::ObjectField {
                    name: "certificateUrls".into(),
                    value: &certificate_urls_binding,
                },
                register_interface::ObjectField {
                    name: "deleteSwgAutogenRouterOnDestroy".into(),
                    value: &delete_swg_autogen_router_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "gatewaySecurityPolicy".into(),
                    value: &gateway_security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
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
                    name: "ports".into(),
                    value: &ports_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "routingMode".into(),
                    value: &routing_mode_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "serverTlsPolicy".into(),
                    value: &server_tls_policy_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GatewayResult {
            addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addresses"),
            ),
            certificate_urls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateUrls"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_swg_autogen_router_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteSwgAutogenRouterOnDestroy"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            gateway_security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewaySecurityPolicy"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            ports: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ports")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            routing_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingMode"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            server_tls_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverTlsPolicy"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
