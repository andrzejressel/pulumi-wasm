/// Manages a LoadBalancer Probe Resource.
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
///     let exampleProbe = probe::create(
///         "exampleProbe",
///         ProbeArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("ssh-running-probe")
///             .port(22)
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
/// Load Balancer Probes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/probe:Probe example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/probes/probe1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod probe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProbeArgs {
        /// The interval, in seconds between probes to the backend endpoint for health status. The default value is 15, the minimum value is 5.
        #[builder(into, default)]
        pub interval_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the LoadBalancer in which to create the Probe. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Probe. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of failed probe attempts after which the backend endpoint is removed from rotation. Default to `2`. NumberOfProbes multiplied by intervalInSeconds value must be greater or equal to 10.Endpoints are returned to rotation when at least one probe is successful.
        #[builder(into, default)]
        pub number_of_probes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Port on which the Probe queries the backend endpoint. Possible values range from 1 to 65535, inclusive.
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The number of consecutive successful or failed probes that allow or deny traffic to this endpoint. Possible values range from `1` to `100`. The default value is `1`.
        #[builder(into, default)]
        pub probe_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the protocol of the end point. Possible values are `Http`, `Https` or `Tcp`. If TCP is specified, a received ACK is required for the probe to be successful. If HTTP is specified, a 200 OK response from the specified URI is required for the probe to be successful. Defaults to `Tcp`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URI used for requesting health status from the backend endpoint. Required if protocol is set to `Http` or `Https`. Otherwise, it is not allowed.
        #[builder(into, default)]
        pub request_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProbeResult {
        /// The interval, in seconds between probes to the backend endpoint for health status. The default value is 15, the minimum value is 5.
        pub interval_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        pub load_balancer_rules: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the LoadBalancer in which to create the Probe. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Probe. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of failed probe attempts after which the backend endpoint is removed from rotation. Default to `2`. NumberOfProbes multiplied by intervalInSeconds value must be greater or equal to 10.Endpoints are returned to rotation when at least one probe is successful.
        pub number_of_probes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Port on which the Probe queries the backend endpoint. Possible values range from 1 to 65535, inclusive.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The number of consecutive successful or failed probes that allow or deny traffic to this endpoint. Possible values range from `1` to `100`. The default value is `1`.
        pub probe_threshold: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the protocol of the end point. Possible values are `Http`, `Https` or `Tcp`. If TCP is specified, a received ACK is required for the probe to be successful. If HTTP is specified, a 200 OK response from the specified URI is required for the probe to be successful. Defaults to `Tcp`.
        pub protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI used for requesting health status from the backend endpoint. Required if protocol is set to `Http` or `Https`. Otherwise, it is not allowed.
        pub request_path: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProbeArgs,
    ) -> ProbeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let interval_in_seconds_binding = args.interval_in_seconds.get_output(context);
        let loadbalancer_id_binding = args.loadbalancer_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let number_of_probes_binding = args.number_of_probes.get_output(context);
        let port_binding = args.port.get_output(context);
        let probe_threshold_binding = args.probe_threshold.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let request_path_binding = args.request_path.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:lb/probe:Probe".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "intervalInSeconds".into(),
                    value: &interval_in_seconds_binding.drop_type(),
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
                    name: "numberOfProbes".into(),
                    value: &number_of_probes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "probeThreshold".into(),
                    value: &probe_threshold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestPath".into(),
                    value: &request_path_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProbeResult {
            interval_in_seconds: o.get_field("intervalInSeconds"),
            load_balancer_rules: o.get_field("loadBalancerRules"),
            loadbalancer_id: o.get_field("loadbalancerId"),
            name: o.get_field("name"),
            number_of_probes: o.get_field("numberOfProbes"),
            port: o.get_field("port"),
            probe_threshold: o.get_field("probeThreshold"),
            protocol: o.get_field("protocol"),
            request_path: o.get_field("requestPath"),
        }
    }
}
