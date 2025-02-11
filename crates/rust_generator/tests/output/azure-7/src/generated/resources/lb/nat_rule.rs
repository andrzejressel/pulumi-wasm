/// Manages a Load Balancer NAT Rule.
///
/// > **NOTE:** This resource cannot be used with with virtual machine scale sets, instead use the `azure.lb.NatPool` resource.
///
/// > **NOTE** When using this resource, the Load Balancer needs to have a FrontEnd IP Configuration Attached
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
///     let example1 = nat_rule::create(
///         "example1",
///         NatRuleArgs::builder()
///             .backend_address_pool_id("${exampleBackendAddressPool.id}")
///             .backend_port(3389)
///             .frontend_ip_configuration_name("PublicIPAddress")
///             .frontend_port_end(3389)
///             .frontend_port_start(3000)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("RDPAccess")
///             .protocol("Tcp")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleBackendAddressPool = backend_address_pool::create(
///         "exampleBackendAddressPool",
///         BackendAddressPoolArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("be")
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
///             .location("West US")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNatRule = nat_rule::create(
///         "exampleNatRule",
///         NatRuleArgs::builder()
///             .backend_port(3389)
///             .frontend_ip_configuration_name("PublicIPAddress")
///             .frontend_port(3389)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("RDPAccess")
///             .protocol("Tcp")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("West US")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancer NAT Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/natRule:NatRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/inboundNatRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nat_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatRuleArgs {
        /// Specifies a reference to backendAddressPool resource.
        #[builder(into, default)]
        pub backend_address_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port used for internal connections on the endpoint. Possible values range between 1 and 65535, inclusive.
        #[builder(into)]
        pub backend_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Are the Floating IPs enabled for this Load Balancer Rule? A "floating” IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group. Defaults to `false`.
        #[builder(into, default)]
        pub enable_floating_ip: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        #[builder(into, default)]
        pub enable_tcp_reset: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the frontend IP configuration exposing this rule.
        #[builder(into)]
        pub frontend_ip_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port for the external endpoint. Port numbers for each Rule must be unique within the Load Balancer. Possible values range between 1 and 65534, inclusive.
        #[builder(into, default)]
        pub frontend_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The port range end for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeStart. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534, inclusive.
        #[builder(into, default)]
        pub frontend_port_end: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The port range start for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeEnd. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534, inclusive.
        #[builder(into, default)]
        pub frontend_port_start: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `30` minutes. Defaults to `4` minutes.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Load Balancer in which to create the NAT Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the NAT Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The transport protocol for the external endpoint. Possible values are `Udp`, `Tcp` or `All`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NatRuleResult {
        /// Specifies a reference to backendAddressPool resource.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub backend_ip_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The port used for internal connections on the endpoint. Possible values range between 1 and 65535, inclusive.
        pub backend_port: pulumi_gestalt_rust::Output<i32>,
        /// Are the Floating IPs enabled for this Load Balancer Rule? A "floating” IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group. Defaults to `false`.
        pub enable_floating_ip: pulumi_gestalt_rust::Output<bool>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        pub enable_tcp_reset: pulumi_gestalt_rust::Output<Option<bool>>,
        pub frontend_ip_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the frontend IP configuration exposing this rule.
        pub frontend_ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The port for the external endpoint. Port numbers for each Rule must be unique within the Load Balancer. Possible values range between 1 and 65534, inclusive.
        pub frontend_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The port range end for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeStart. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534, inclusive.
        pub frontend_port_end: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The port range start for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeEnd. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534, inclusive.
        pub frontend_port_start: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `30` minutes. Defaults to `4` minutes.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Load Balancer in which to create the NAT Rule. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the NAT Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The transport protocol for the external endpoint. Possible values are `Udp`, `Tcp` or `All`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NatRuleArgs,
    ) -> NatRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_address_pool_id_binding = args
            .backend_address_pool_id
            .get_output(context);
        let backend_port_binding = args.backend_port.get_output(context);
        let enable_floating_ip_binding = args.enable_floating_ip.get_output(context);
        let enable_tcp_reset_binding = args.enable_tcp_reset.get_output(context);
        let frontend_ip_configuration_name_binding = args
            .frontend_ip_configuration_name
            .get_output(context);
        let frontend_port_binding = args.frontend_port.get_output(context);
        let frontend_port_end_binding = args.frontend_port_end.get_output(context);
        let frontend_port_start_binding = args.frontend_port_start.get_output(context);
        let idle_timeout_in_minutes_binding = args
            .idle_timeout_in_minutes
            .get_output(context);
        let loadbalancer_id_binding = args.loadbalancer_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:lb/natRule:NatRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendAddressPoolId".into(),
                    value: &backend_address_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendPort".into(),
                    value: &backend_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableFloatingIp".into(),
                    value: &enable_floating_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTcpReset".into(),
                    value: &enable_tcp_reset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendIpConfigurationName".into(),
                    value: &frontend_ip_configuration_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPort".into(),
                    value: &frontend_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPortEnd".into(),
                    value: &frontend_port_end_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPortStart".into(),
                    value: &frontend_port_start_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: &idle_timeout_in_minutes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NatRuleResult {
            backend_address_pool_id: o.get_field("backendAddressPoolId"),
            backend_ip_configuration_id: o.get_field("backendIpConfigurationId"),
            backend_port: o.get_field("backendPort"),
            enable_floating_ip: o.get_field("enableFloatingIp"),
            enable_tcp_reset: o.get_field("enableTcpReset"),
            frontend_ip_configuration_id: o.get_field("frontendIpConfigurationId"),
            frontend_ip_configuration_name: o.get_field("frontendIpConfigurationName"),
            frontend_port: o.get_field("frontendPort"),
            frontend_port_end: o.get_field("frontendPortEnd"),
            frontend_port_start: o.get_field("frontendPortStart"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            loadbalancer_id: o.get_field("loadbalancerId"),
            name: o.get_field("name"),
            protocol: o.get_field("protocol"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
