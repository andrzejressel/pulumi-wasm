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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// Sets an alternative name server for the associated networks.
        /// When specified, all DNS queries are forwarded to a name server that you choose.
        /// Names such as .internal are not available when an alternative name server is specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub alternative_name_server_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::PolicyAlternativeNameServerConfig>,
        >,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allows networks bound to this policy to receive DNS queries sent
        /// by VMs or applications over VPN connections. When enabled, a
        /// virtual IP address will be allocated from each of the sub-networks
        /// that are bound to this policy.
        #[builder(into, default)]
        pub enable_inbound_forwarding: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Controls whether logging is enabled for the networks bound to this policy.
        /// Defaults to no logging if not set.
        #[builder(into, default)]
        pub enable_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User assigned name for this policy.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dns::PolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// Sets an alternative name server for the associated networks.
        /// When specified, all DNS queries are forwarded to a name server that you choose.
        /// Names such as .internal are not available when an alternative name server is specified.
        /// Structure is documented below.
        pub alternative_name_server_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dns::PolicyAlternativeNameServerConfig>,
        >,
        /// A textual description field. Defaults to 'Managed by Pulumi'.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Allows networks bound to this policy to receive DNS queries sent
        /// by VMs or applications over VPN connections. When enabled, a
        /// virtual IP address will be allocated from each of the sub-networks
        /// that are bound to this policy.
        pub enable_inbound_forwarding: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Controls whether logging is enabled for the networks bound to this policy.
        /// Defaults to no logging if not set.
        pub enable_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User assigned name for this policy.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of network names specifying networks to which this policy is applied.
        /// Structure is documented below.
        pub networks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dns::PolicyNetwork>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alternative_name_server_config_binding = args
            .alternative_name_server_config
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let enable_inbound_forwarding_binding = args
            .enable_inbound_forwarding
            .get_output(context);
        let enable_logging_binding = args.enable_logging.get_output(context);
        let name_binding = args.name.get_output(context);
        let networks_binding = args.networks.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dns/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alternativeNameServerConfig".into(),
                    value: alternative_name_server_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableInboundForwarding".into(),
                    value: enable_inbound_forwarding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableLogging".into(),
                    value: enable_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networks".into(),
                    value: networks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            alternative_name_server_config: o.get_field("alternativeNameServerConfig"),
            description: o.get_field("description"),
            enable_inbound_forwarding: o.get_field("enableInboundForwarding"),
            enable_logging: o.get_field("enableLogging"),
            name: o.get_field("name"),
            networks: o.get_field("networks"),
            project: o.get_field("project"),
        }
    }
}
