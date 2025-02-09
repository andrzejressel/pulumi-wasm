/// Provides a proxy protocol policy, which allows an ELB to carry a client connection information to a backend.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lb = load_balancer::create(
///         "lb",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(25)
///                     .instanceProtocol("tcp").lbPort(25).lbProtocol("tcp").build_struct(),
///                     LoadBalancerListener::builder().instancePort(587)
///                     .instanceProtocol("tcp").lbPort(587).lbProtocol("tcp")
///                     .build_struct(),
///                 ],
///             )
///             .name("test-lb")
///             .build_struct(),
///     );
///     let smtp = proxy_protocol_policy::create(
///         "smtp",
///         ProxyProtocolPolicyArgs::builder()
///             .instance_ports(vec!["25", "587",])
///             .load_balancer("${lb.name}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod proxy_protocol_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyArgs {
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        #[builder(into)]
        pub instance_ports: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyResult {
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        pub instance_ports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyProtocolPolicyArgs,
    ) -> ProxyProtocolPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_ports_binding = args.instance_ports.get_output(context);
        let load_balancer_binding = args.load_balancer.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/proxyProtocolPolicy:ProxyProtocolPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instancePorts".into(),
                    value: instance_ports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: load_balancer_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProxyProtocolPolicyResult {
            instance_ports: o.get_field("instancePorts"),
            load_balancer: o.get_field("loadBalancer"),
        }
    }
}
