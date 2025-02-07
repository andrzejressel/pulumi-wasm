pub mod get_network_insights_path {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInsightsPathArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetNetworkInsightsPathFilter>>,
        >,
        /// ID of the Network Insights Path to select.
        #[builder(into, default)]
        pub network_insights_path_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInsightsPathResult {
        /// ARN of the selected Network Insights Path.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS resource that is the destination of the path.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// ARN of the destination.
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// IP address of the AWS resource that is the destination of the path.
        pub destination_ip: pulumi_gestalt_rust::Output<String>,
        /// Destination port.
        pub destination_port: pulumi_gestalt_rust::Output<i32>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNetworkInsightsPathFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub network_insights_path_id: pulumi_gestalt_rust::Output<String>,
        /// Protocol.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// AWS resource that is the source of the path.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// ARN of the source.
        pub source_arn: pulumi_gestalt_rust::Output<String>,
        /// IP address of the AWS resource that is the source of the path.
        pub source_ip: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkInsightsPathArgs,
    ) -> GetNetworkInsightsPathResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let network_insights_path_id_binding = args
            .network_insights_path_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNetworkInsightsPath:getNetworkInsightsPath".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "networkInsightsPathId".into(),
                    value: &network_insights_path_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkInsightsPathResult {
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
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            network_insights_path_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInsightsPathId"),
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
        }
    }
}
