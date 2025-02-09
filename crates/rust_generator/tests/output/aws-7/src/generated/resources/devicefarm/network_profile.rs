/// Provides a resource to manage AWS Device Farm Network Profiles.
/// ∂
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder().name("example").build_struct(),
///     );
///     let exampleNetworkProfile = network_profile::create(
///         "exampleNetworkProfile",
///         NetworkProfileArgs::builder()
///             .name("example")
///             .project_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Network Profiles using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/networkProfile:NetworkProfile example arn:aws:devicefarm:us-west-2:123456789012:networkprofile:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkProfileArgs {
        /// The description of the network profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The data throughput rate in bits per second, as an integer from `0` to `104857600`. Default value is `104857600`.
        #[builder(into, default)]
        pub downlink_bandwidth_bits: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Delay time for all packets to destination in milliseconds as an integer from `0` to `2000`.
        #[builder(into, default)]
        pub downlink_delay_ms: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Time variation in the delay of received packets in milliseconds as an integer from `0` to `2000`.
        #[builder(into, default)]
        pub downlink_jitter_ms: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Proportion of received packets that fail to arrive from `0` to `100` percent.
        #[builder(into, default)]
        pub downlink_loss_percent: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name for the network profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the project for the network profile.
        #[builder(into)]
        pub project_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of network profile to create. Valid values are listed are `PRIVATE` and `CURATED`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The data throughput rate in bits per second, as an integer from `0` to `104857600`. Default value is `104857600`.
        #[builder(into, default)]
        pub uplink_bandwidth_bits: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Delay time for all packets to destination in milliseconds as an integer from `0` to `2000`.
        #[builder(into, default)]
        pub uplink_delay_ms: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Time variation in the delay of received packets in milliseconds as an integer from `0` to `2000`.
        #[builder(into, default)]
        pub uplink_jitter_ms: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Proportion of received packets that fail to arrive from `0` to `100` percent.
        #[builder(into, default)]
        pub uplink_loss_percent: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct NetworkProfileResult {
        /// The Amazon Resource Name of this network profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the network profile.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The data throughput rate in bits per second, as an integer from `0` to `104857600`. Default value is `104857600`.
        pub downlink_bandwidth_bits: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Delay time for all packets to destination in milliseconds as an integer from `0` to `2000`.
        pub downlink_delay_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Time variation in the delay of received packets in milliseconds as an integer from `0` to `2000`.
        pub downlink_jitter_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Proportion of received packets that fail to arrive from `0` to `100` percent.
        pub downlink_loss_percent: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name for the network profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the project for the network profile.
        pub project_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of network profile to create. Valid values are listed are `PRIVATE` and `CURATED`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The data throughput rate in bits per second, as an integer from `0` to `104857600`. Default value is `104857600`.
        pub uplink_bandwidth_bits: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Delay time for all packets to destination in milliseconds as an integer from `0` to `2000`.
        pub uplink_delay_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Time variation in the delay of received packets in milliseconds as an integer from `0` to `2000`.
        pub uplink_jitter_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Proportion of received packets that fail to arrive from `0` to `100` percent.
        pub uplink_loss_percent: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkProfileArgs,
    ) -> NetworkProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let downlink_bandwidth_bits_binding = args
            .downlink_bandwidth_bits
            .get_output(context);
        let downlink_delay_ms_binding = args.downlink_delay_ms.get_output(context);
        let downlink_jitter_ms_binding = args.downlink_jitter_ms.get_output(context);
        let downlink_loss_percent_binding = args
            .downlink_loss_percent
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_arn_binding = args.project_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let uplink_bandwidth_bits_binding = args
            .uplink_bandwidth_bits
            .get_output(context);
        let uplink_delay_ms_binding = args.uplink_delay_ms.get_output(context);
        let uplink_jitter_ms_binding = args.uplink_jitter_ms.get_output(context);
        let uplink_loss_percent_binding = args.uplink_loss_percent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devicefarm/networkProfile:NetworkProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "downlinkBandwidthBits".into(),
                    value: downlink_bandwidth_bits_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "downlinkDelayMs".into(),
                    value: downlink_delay_ms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "downlinkJitterMs".into(),
                    value: downlink_jitter_ms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "downlinkLossPercent".into(),
                    value: downlink_loss_percent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectArn".into(),
                    value: project_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uplinkBandwidthBits".into(),
                    value: uplink_bandwidth_bits_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uplinkDelayMs".into(),
                    value: uplink_delay_ms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uplinkJitterMs".into(),
                    value: uplink_jitter_ms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uplinkLossPercent".into(),
                    value: uplink_loss_percent_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkProfileResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            downlink_bandwidth_bits: o.get_field("downlinkBandwidthBits"),
            downlink_delay_ms: o.get_field("downlinkDelayMs"),
            downlink_jitter_ms: o.get_field("downlinkJitterMs"),
            downlink_loss_percent: o.get_field("downlinkLossPercent"),
            name: o.get_field("name"),
            project_arn: o.get_field("projectArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            uplink_bandwidth_bits: o.get_field("uplinkBandwidthBits"),
            uplink_delay_ms: o.get_field("uplinkDelayMs"),
            uplink_jitter_ms: o.get_field("uplinkJitterMs"),
            uplink_loss_percent: o.get_field("uplinkLossPercent"),
        }
    }
}
