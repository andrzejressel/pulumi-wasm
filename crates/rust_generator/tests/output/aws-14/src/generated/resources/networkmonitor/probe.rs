/// Resource for managing an AWS Network Monitor Probe.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = monitor::create(
///         "example",
///         MonitorArgs::builder()
///             .aggregation_period(30)
///             .monitor_name("example")
///             .build_struct(),
///     );
///     let exampleProbe = probe::create(
///         "exampleProbe",
///         ProbeArgs::builder()
///             .destination("127.0.0.1")
///             .destination_port(80)
///             .monitor_name("${example.monitorName}")
///             .packet_size(200)
///             .protocol("TCP")
///             .source_arn("${exampleAwsSubnet.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmonitor_probe` using the monitor name and probe id. For example:
///
/// ```sh
/// $ pulumi import aws:networkmonitor/probe:Probe example monitor-7786087912324693644,probe-3qm8p693i4fi1h8lqylzkbp42e
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod probe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProbeArgs {
        /// The destination IP address. This must be either IPV4 or IPV6.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port associated with the destination. This is required only if the protocol is TCP and must be a number between 1 and 65536.
        #[builder(into, default)]
        pub destination_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the monitor.
        #[builder(into)]
        pub monitor_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of the packets sent between the source and destination. This must be a number between 56 and 8500.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub packet_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The protocol used for the network traffic between the source and destination. This must be either TCP or ICMP.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the subnet.
        #[builder(into)]
        pub source_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProbeResult {
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The destination IP address. This must be either IPV4 or IPV6.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// The port associated with the destination. This is required only if the protocol is TCP and must be a number between 1 and 65536.
        pub destination_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the monitor.
        pub monitor_name: pulumi_gestalt_rust::Output<String>,
        /// The size of the packets sent between the source and destination. This must be a number between 56 and 8500.
        ///
        /// The following arguments are optional:
        pub packet_size: pulumi_gestalt_rust::Output<i32>,
        pub probe_id: pulumi_gestalt_rust::Output<String>,
        /// The protocol used for the network traffic between the source and destination. This must be either TCP or ICMP.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the subnet.
        pub source_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_binding = args.destination.get_output(context);
        let destination_port_binding = args.destination_port.get_output(context);
        let monitor_name_binding = args.monitor_name.get_output(context);
        let packet_size_binding = args.packet_size.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let source_arn_binding = args.source_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmonitor/probe:Probe".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationPort".into(),
                    value: destination_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitorName".into(),
                    value: monitor_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packetSize".into(),
                    value: packet_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceArn".into(),
                    value: source_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProbeResult {
            address_family: o.get_field("addressFamily"),
            arn: o.get_field("arn"),
            destination: o.get_field("destination"),
            destination_port: o.get_field("destinationPort"),
            monitor_name: o.get_field("monitorName"),
            packet_size: o.get_field("packetSize"),
            probe_id: o.get_field("probeId"),
            protocol: o.get_field("protocol"),
            source_arn: o.get_field("sourceArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
