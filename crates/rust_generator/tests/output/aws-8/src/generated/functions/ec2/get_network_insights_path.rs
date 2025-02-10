#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkInsightsPathArgs,
    ) -> GetNetworkInsightsPathResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let network_insights_path_id_binding = args
            .network_insights_path_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getNetworkInsightsPath:getNetworkInsightsPath".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInsightsPathId".into(),
                    value: network_insights_path_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkInsightsPathResult {
            arn: o.get_field("arn"),
            destination: o.get_field("destination"),
            destination_arn: o.get_field("destinationArn"),
            destination_ip: o.get_field("destinationIp"),
            destination_port: o.get_field("destinationPort"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            network_insights_path_id: o.get_field("networkInsightsPathId"),
            protocol: o.get_field("protocol"),
            source: o.get_field("source"),
            source_arn: o.get_field("sourceArn"),
            source_ip: o.get_field("sourceIp"),
            tags: o.get_field("tags"),
        }
    }
}
