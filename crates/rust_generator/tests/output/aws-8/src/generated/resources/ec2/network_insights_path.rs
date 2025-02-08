/// Provides a Network Insights Path resource. Part of the "Reachability Analyzer" service in the AWS VPC console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod network_insights_path {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInsightsPathArgs {
        /// ID or ARN of the resource which is the destination of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        #[builder(into, default)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address of the destination resource.
        #[builder(into, default)]
        pub destination_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Destination port to analyze access to.
        #[builder(into, default)]
        pub destination_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Protocol to use for analysis. Valid options are `tcp` or `udp`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID or ARN of the resource which is the source of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// IP address of the source resource.
        #[builder(into, default)]
        pub source_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkInsightsPathResult {
        /// ARN of the Network Insights Path.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID or ARN of the resource which is the destination of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        pub destination: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the destination.
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// IP address of the destination resource.
        pub destination_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// Destination port to analyze access to.
        pub destination_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Protocol to use for analysis. Valid options are `tcp` or `udp`.
        ///
        /// The following arguments are optional:
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// ID or ARN of the resource which is the source of the path. Can be an Instance, Internet Gateway, Network Interface, Transit Gateway, VPC Endpoint, VPC Peering Connection or VPN Gateway. If the resource is in another account, you must specify an ARN.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// ARN of the source.
        pub source_arn: pulumi_gestalt_rust::Output<String>,
        /// IP address of the source resource.
        pub source_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInsightsPathArgs,
    ) -> NetworkInsightsPathResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_binding = args.destination.get_output(context).get_inner();
        let destination_ip_binding = args.destination_ip.get_output(context).get_inner();
        let destination_port_binding = args
            .destination_port
            .get_output(context)
            .get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let source_ip_binding = args.source_ip.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInsightsPathResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            destination_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationArn"),
            ),
            destination_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationIp"),
            ),
            destination_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationPort"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            source_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceArn"),
            ),
            source_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIp"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
