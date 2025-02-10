/// Manages a Load Balancer Outbound Rule.
///
/// > **NOTE** When using this resource, the Load Balancer needs to have a FrontEnd IP Configuration and a Backend Address Pool Attached.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("LoadBalancerRG")
///             .build_struct(),
///     );
///     let exampleBackendAddressPool = backend_address_pool::create(
///         "exampleBackendAddressPool",
///         BackendAddressPoolArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder()
///                     .name("PublicIPAddress").publicIpAddressId("${examplePublicIp.id}")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleOutboundRule = outbound_rule::create(
///         "exampleOutboundRule",
///         OutboundRuleArgs::builder()
///             .backend_address_pool_id("${exampleBackendAddressPool.id}")
///             .frontend_ip_configurations(
///                 vec![
///                     OutboundRuleFrontendIpConfiguration::builder()
///                     .name("PublicIPAddress").build_struct(),
///                 ],
///             )
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("OutboundRule")
///             .protocol("Tcp")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancer Outbound Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/outboundRule:OutboundRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/outboundRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod outbound_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutboundRuleArgs {
        /// The number of outbound ports to be used for NAT. Defaults to `1024`.
        #[builder(into, default)]
        pub allocated_outbound_ports: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Backend Address Pool. Outbound traffic is randomly load balanced across IPs in the backend IPs.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
        #[builder(into, default)]
        pub enable_tcp_reset: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        #[builder(into, default)]
        pub frontend_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lb::OutboundRuleFrontendIpConfiguration>>,
        >,
        /// The timeout for the TCP idle connection Defaults to `4`.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Load Balancer in which to create the Outbound Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Outbound Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The transport protocol for the external endpoint. Possible values are `Udp`, `Tcp` or `All`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutboundRuleResult {
        /// The number of outbound ports to be used for NAT. Defaults to `1024`.
        pub allocated_outbound_ports: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Backend Address Pool. Outbound traffic is randomly load balanced across IPs in the backend IPs.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
        pub enable_tcp_reset: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `frontend_ip_configuration` blocks as defined below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lb::OutboundRuleFrontendIpConfiguration>>,
        >,
        /// The timeout for the TCP idle connection Defaults to `4`.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Load Balancer in which to create the Outbound Rule. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Outbound Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The transport protocol for the external endpoint. Possible values are `Udp`, `Tcp` or `All`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OutboundRuleArgs,
    ) -> OutboundRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocated_outbound_ports_binding = args
            .allocated_outbound_ports
            .get_output(context);
        let backend_address_pool_id_binding = args
            .backend_address_pool_id
            .get_output(context);
        let enable_tcp_reset_binding = args.enable_tcp_reset.get_output(context);
        let frontend_ip_configurations_binding = args
            .frontend_ip_configurations
            .get_output(context);
        let idle_timeout_in_minutes_binding = args
            .idle_timeout_in_minutes
            .get_output(context);
        let loadbalancer_id_binding = args.loadbalancer_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:lb/outboundRule:OutboundRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocatedOutboundPorts".into(),
                    value: allocated_outbound_ports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendAddressPoolId".into(),
                    value: backend_address_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTcpReset".into(),
                    value: enable_tcp_reset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendIpConfigurations".into(),
                    value: frontend_ip_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: idle_timeout_in_minutes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadbalancerId".into(),
                    value: loadbalancer_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OutboundRuleResult {
            allocated_outbound_ports: o.get_field("allocatedOutboundPorts"),
            backend_address_pool_id: o.get_field("backendAddressPoolId"),
            enable_tcp_reset: o.get_field("enableTcpReset"),
            frontend_ip_configurations: o.get_field("frontendIpConfigurations"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            loadbalancer_id: o.get_field("loadbalancerId"),
            name: o.get_field("name"),
            protocol: o.get_field("protocol"),
        }
    }
}
