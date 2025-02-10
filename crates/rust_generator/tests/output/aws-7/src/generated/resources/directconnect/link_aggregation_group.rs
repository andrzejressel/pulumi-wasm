/// Provides a Direct Connect LAG. Connections can be added to the LAG via the `aws.directconnect.Connection` and `aws.directconnect.ConnectionAssociation` resources.
///
/// > *NOTE:* When creating a LAG, if no existing connection is specified, Direct Connect will create a connection and this provider will remove this unmanaged connection during resource creation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hoge = link_aggregation_group::create(
///         "hoge",
///         LinkAggregationGroupArgs::builder()
///             .connections_bandwidth("1Gbps")
///             .force_destroy(true)
///             .location("EqDC2")
///             .name("tf-dx-lag")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect LAGs using the LAG `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/linkAggregationGroup:LinkAggregationGroup test_lag dxlag-fgnsp5rq
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod link_aggregation_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkAggregationGroupArgs {
        /// The ID of an existing dedicated connection to migrate to the LAG.
        #[builder(into, default)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The bandwidth of the individual physical connections bundled by the LAG. Valid values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        #[builder(into)]
        pub connections_bandwidth: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A boolean that indicates all connections associated with the LAG should be deleted so that the LAG can be destroyed without error. These objects are *not* recoverable.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The AWS Direct Connect location in which the LAG should be allocated. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the LAG.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the service provider associated with the LAG.
        #[builder(into, default)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkAggregationGroupResult {
        /// The ARN of the LAG.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of an existing dedicated connection to migrate to the LAG.
        pub connection_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The bandwidth of the individual physical connections bundled by the LAG. Valid values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        pub connections_bandwidth: pulumi_gestalt_rust::Output<String>,
        /// A boolean that indicates all connections associated with the LAG should be deleted so that the LAG can be destroyed without error. These objects are *not* recoverable.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether the LAG supports a secondary BGP peer in the same address family (IPv4/IPv6).
        pub has_logical_redundancy: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether jumbo frames (9001 MTU) are supported.
        pub jumbo_frame_capable: pulumi_gestalt_rust::Output<bool>,
        /// The AWS Direct Connect location in which the LAG should be allocated. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the LAG.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the LAG.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the service provider associated with the LAG.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkAggregationGroupArgs,
    ) -> LinkAggregationGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_id_binding = args.connection_id.get_output(context);
        let connections_bandwidth_binding = args
            .connections_bandwidth
            .get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/linkAggregationGroup:LinkAggregationGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: connection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionsBandwidth".into(),
                    value: connections_bandwidth_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkAggregationGroupResult {
            arn: o.get_field("arn"),
            connection_id: o.get_field("connectionId"),
            connections_bandwidth: o.get_field("connectionsBandwidth"),
            force_destroy: o.get_field("forceDestroy"),
            has_logical_redundancy: o.get_field("hasLogicalRedundancy"),
            jumbo_frame_capable: o.get_field("jumboFrameCapable"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
            provider_name: o.get_field("providerName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
