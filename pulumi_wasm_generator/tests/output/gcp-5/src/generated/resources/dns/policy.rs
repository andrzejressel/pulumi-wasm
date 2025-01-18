/// A policy is a collection of DNS rules applied to one or more Virtual
/// Private Cloud resources.
///
///
/// To get more information about Policy, see:
///
/// * [API documentation](https://cloud.google.com/dns/docs/reference/v1beta2/policies)
/// * How-to Guides
///     * [Using DNS server policies](https://cloud.google.com/dns/zones/#using-dns-server-policies)
///
/// ## Example Usage
///
/// ### Dns Policy Basic
///
///
/// ```yaml
/// resources:
///   example-policy:
///     type: gcp:dns:Policy
///     properties:
///       name: example-policy
///       enableInboundForwarding: true
///       enableLogging: true
///       alternativeNameServerConfig:
///         targetNameServers:
///           - ipv4Address: 172.16.1.10
///             forwardingPath: private
///           - ipv4Address: 172.16.1.20
///       networks:
///         - networkUrl: ${["network-1"].id}
///         - networkUrl: ${["network-2"].id}
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// Policy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/policies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Policy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/policy:Policy default projects/{{project}}/policies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/policy:Policy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/policy:Policy default {{name}}
/// ```
///
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Sets an alternative name server for the associated networks.
        /// When specified, all DNS queries are forwarded to a name server that you choose.
        /// Names such as .internal are not available when an alternative name server is specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub alternative_name_server_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::PolicyAlternativeNameServerConfig>,
        >,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Allows networks bound to this policy to receive DNS queries sent
        /// by VMs or applications over VPN connections. When enabled, a
        /// virtual IP address will be allocated from each of the sub-networks
        /// that are bound to this policy.
        #[builder(into, default)]
        pub enable_inbound_forwarding: pulumi_wasm_rust::Output<Option<bool>>,
        /// Controls whether logging is enabled for the networks bound to this policy.
        /// Defaults to no logging if not set.
        #[builder(into, default)]
        pub enable_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// User assigned name for this policy.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dns::PolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// Sets an alternative name server for the associated networks.
        /// When specified, all DNS queries are forwarded to a name server that you choose.
        /// Names such as .internal are not available when an alternative name server is specified.
        /// Structure is documented below.
        pub alternative_name_server_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::PolicyAlternativeNameServerConfig>,
        >,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Allows networks bound to this policy to receive DNS queries sent
        /// by VMs or applications over VPN connections. When enabled, a
        /// virtual IP address will be allocated from each of the sub-networks
        /// that are bound to this policy.
        pub enable_inbound_forwarding: pulumi_wasm_rust::Output<Option<bool>>,
        /// Controls whether logging is enabled for the networks bound to this policy.
        /// Defaults to no logging if not set.
        pub enable_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// User assigned name for this policy.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        pub networks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dns::PolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alternative_name_server_config_binding = args
            .alternative_name_server_config
            .get_inner();
        let description_binding = args.description.get_inner();
        let enable_inbound_forwarding_binding = args
            .enable_inbound_forwarding
            .get_inner();
        let enable_logging_binding = args.enable_logging.get_inner();
        let name_binding = args.name.get_inner();
        let networks_binding = args.networks.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dns/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alternativeNameServerConfig".into(),
                    value: &alternative_name_server_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enableInboundForwarding".into(),
                    value: &enable_inbound_forwarding_binding,
                },
                register_interface::ObjectField {
                    name: "enableLogging".into(),
                    value: &enable_logging_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networks".into(),
                    value: &networks_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternativeNameServerConfig".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enableInboundForwarding".into(),
                },
                register_interface::ResultField {
                    name: "enableLogging".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyResult {
            alternative_name_server_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternativeNameServerConfig").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enable_inbound_forwarding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableInboundForwarding").unwrap(),
            ),
            enable_logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableLogging").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
