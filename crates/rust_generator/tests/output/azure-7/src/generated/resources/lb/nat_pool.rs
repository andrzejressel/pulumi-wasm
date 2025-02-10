/// Manages a Load Balancer NAT pool.
///
/// > **NOTE:** This resource cannot be used with with virtual machines, instead use the `azure.lb.NatRule` resource.
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
///     let exampleNatPool = nat_pool::create(
///         "exampleNatPool",
///         NatPoolArgs::builder()
///             .backend_port(8080)
///             .frontend_ip_configuration_name("PublicIPAddress")
///             .frontend_port_end(81)
///             .frontend_port_start(80)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("SampleApplicationPool")
///             .protocol("Tcp")
///             .resource_group_name("${example.name}")
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
/// Load Balancer NAT Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/natPool:NatPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/inboundNatPools/pool1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nat_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatPoolArgs {
        /// The port used for the internal endpoint. Possible values range between 1 and 65535, inclusive.
        #[builder(into)]
        pub backend_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Are the floating IPs enabled for this Load Balancer Rule? A floating IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group.
        #[builder(into, default)]
        pub floating_ip_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the frontend IP configuration exposing this rule.
        #[builder(into)]
        pub frontend_ip_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The last port number in the range of external ports that will be used to provide Inbound NAT to NICs associated with this Load Balancer. Possible values range between 1 and 65534, inclusive.
        #[builder(into)]
        pub frontend_port_end: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The first port number in the range of external ports that will be used to provide Inbound NAT to NICs associated with this Load Balancer. Possible values range between 1 and 65534, inclusive.
        #[builder(into)]
        pub frontend_port_start: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `30`. Defaults to `4`.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Load Balancer in which to create the NAT pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the NAT pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The transport protocol for the external endpoint. Possible values are `All`, `Tcp` and `Udp`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        #[builder(into, default)]
        pub tcp_reset_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NatPoolResult {
        /// The port used for the internal endpoint. Possible values range between 1 and 65535, inclusive.
        pub backend_port: pulumi_gestalt_rust::Output<i32>,
        /// Are the floating IPs enabled for this Load Balancer Rule? A floating IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group.
        pub floating_ip_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub frontend_ip_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the frontend IP configuration exposing this rule.
        pub frontend_ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The last port number in the range of external ports that will be used to provide Inbound NAT to NICs associated with this Load Balancer. Possible values range between 1 and 65534, inclusive.
        pub frontend_port_end: pulumi_gestalt_rust::Output<i32>,
        /// The first port number in the range of external ports that will be used to provide Inbound NAT to NICs associated with this Load Balancer. Possible values range between 1 and 65534, inclusive.
        pub frontend_port_start: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `30`. Defaults to `4`.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Load Balancer in which to create the NAT pool. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the NAT pool. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The transport protocol for the external endpoint. Possible values are `All`, `Tcp` and `Udp`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        pub tcp_reset_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NatPoolArgs,
    ) -> NatPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_port_binding = args.backend_port.get_output(context);
        let floating_ip_enabled_binding = args.floating_ip_enabled.get_output(context);
        let frontend_ip_configuration_name_binding = args
            .frontend_ip_configuration_name
            .get_output(context);
        let frontend_port_end_binding = args.frontend_port_end.get_output(context);
        let frontend_port_start_binding = args.frontend_port_start.get_output(context);
        let idle_timeout_in_minutes_binding = args
            .idle_timeout_in_minutes
            .get_output(context);
        let loadbalancer_id_binding = args.loadbalancer_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tcp_reset_enabled_binding = args.tcp_reset_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:lb/natPool:NatPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendPort".into(),
                    value: backend_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "floatingIpEnabled".into(),
                    value: floating_ip_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendIpConfigurationName".into(),
                    value: frontend_ip_configuration_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPortEnd".into(),
                    value: frontend_port_end_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPortStart".into(),
                    value: frontend_port_start_binding.get_id(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tcpResetEnabled".into(),
                    value: tcp_reset_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NatPoolResult {
            backend_port: o.get_field("backendPort"),
            floating_ip_enabled: o.get_field("floatingIpEnabled"),
            frontend_ip_configuration_id: o.get_field("frontendIpConfigurationId"),
            frontend_ip_configuration_name: o.get_field("frontendIpConfigurationName"),
            frontend_port_end: o.get_field("frontendPortEnd"),
            frontend_port_start: o.get_field("frontendPortStart"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            loadbalancer_id: o.get_field("loadbalancerId"),
            name: o.get_field("name"),
            protocol: o.get_field("protocol"),
            resource_group_name: o.get_field("resourceGroupName"),
            tcp_reset_enabled: o.get_field("tcpResetEnabled"),
        }
    }
}
