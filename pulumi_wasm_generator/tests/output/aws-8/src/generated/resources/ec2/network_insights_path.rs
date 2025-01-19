/// Provides a Network Insights Path resource. Part of the "Reachability Analyzer" service in the AWS VPC console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = network_insights_path::create(
///         "test",
///         NetworkInsightsPathArgs::builder()
///             .destination("${destination.id}")
///             .protocol("tcp")
///             .source("${source.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Insights Paths using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInsightsPath:NetworkInsightsPath test nip-00edfba169923aefd
/// ```
pub mod network_insights_path {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInsightsPathArgs {
        /// ID or ARN of the resource which is the destination of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        #[builder(into, default)]
        pub destination: pulumi_wasm_rust::Output<Option<String>>,
        /// IP address of the destination resource.
        #[builder(into, default)]
        pub destination_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination port to analyze access to.
        #[builder(into, default)]
        pub destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Protocol to use for analysis. Valid options are `tcp` or `udp`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// ID or ARN of the resource which is the source of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
        /// IP address of the source resource.
        #[builder(into, default)]
        pub source_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkInsightsPathResult {
        /// ARN of the Network Insights Path.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID or ARN of the resource which is the destination of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        pub destination: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the destination.
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// IP address of the destination resource.
        pub destination_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination port to analyze access to.
        pub destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Protocol to use for analysis. Valid options are `tcp` or `udp`.
        ///
        /// The following arguments are optional:
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// ID or ARN of the resource which is the source of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        pub source: pulumi_wasm_rust::Output<String>,
        /// ARN of the source.
        pub source_arn: pulumi_wasm_rust::Output<String>,
        /// IP address of the source resource.
        pub source_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkInsightsPathArgs,
    ) -> NetworkInsightsPathResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_binding = args.destination.get_inner();
        let destination_ip_binding = args.destination_ip.get_inner();
        let destination_port_binding = args.destination_port.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let source_binding = args.source.get_inner();
        let source_ip_binding = args.source_ip.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInsightsPath:NetworkInsightsPath".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "destinationIp".into(),
                    value: &destination_ip_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPort".into(),
                    value: &destination_port_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "sourceIp".into(),
                    value: &source_ip_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationIp".into(),
                },
                register_interface::ResultField {
                    name: "destinationPort".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "sourceArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceIp".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInsightsPathResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            destination_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationIp").unwrap(),
            ),
            destination_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPort").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArn").unwrap(),
            ),
            source_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIp").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
