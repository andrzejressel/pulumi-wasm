/// Resource for managing an AWS Network Monitor Probe.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod probe {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProbeArgs {
        /// The destination IP address. This must be either IPV4 or IPV6.
        #[builder(into)]
        pub destination: pulumi_wasm_rust::Output<String>,
        /// The port associated with the destination. This is required only if the protocol is TCP and must be a number between 1 and 65536.
        #[builder(into, default)]
        pub destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the monitor.
        #[builder(into)]
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// The size of the packets sent between the source and destination. This must be a number between 56 and 8500.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub packet_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The protocol used for the network traffic between the source and destination. This must be either TCP or ICMP.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The ARN of the subnet.
        #[builder(into)]
        pub source_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProbeResult {
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// The ARN of the attachment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The destination IP address. This must be either IPV4 or IPV6.
        pub destination: pulumi_wasm_rust::Output<String>,
        /// The port associated with the destination. This is required only if the protocol is TCP and must be a number between 1 and 65536.
        pub destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the monitor.
        pub monitor_name: pulumi_wasm_rust::Output<String>,
        /// The size of the packets sent between the source and destination. This must be a number between 56 and 8500.
        ///
        /// The following arguments are optional:
        pub packet_size: pulumi_wasm_rust::Output<i32>,
        pub probe_id: pulumi_wasm_rust::Output<String>,
        /// The protocol used for the network traffic between the source and destination. This must be either TCP or ICMP.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The ARN of the subnet.
        pub source_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the monitor. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProbeArgs) -> ProbeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_binding = args.destination.get_inner();
        let destination_port_binding = args.destination_port.get_inner();
        let monitor_name_binding = args.monitor_name.get_inner();
        let packet_size_binding = args.packet_size.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let source_arn_binding = args.source_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmonitor/probe:Probe".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPort".into(),
                    value: &destination_port_binding,
                },
                register_interface::ObjectField {
                    name: "monitorName".into(),
                    value: &monitor_name_binding,
                },
                register_interface::ObjectField {
                    name: "packetSize".into(),
                    value: &packet_size_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "sourceArn".into(),
                    value: &source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "destinationPort".into(),
                },
                register_interface::ResultField {
                    name: "monitorName".into(),
                },
                register_interface::ResultField {
                    name: "packetSize".into(),
                },
                register_interface::ResultField {
                    name: "probeId".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "sourceArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProbeResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            destination_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPort").unwrap(),
            ),
            monitor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorName").unwrap(),
            ),
            packet_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packetSize").unwrap(),
            ),
            probe_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probeId").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
