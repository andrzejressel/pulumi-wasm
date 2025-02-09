/// Provides an Traffic mirror session.
/// Read [limits and considerations](https://docs.aws.amazon.com/vpc/latest/mirroring/traffic-mirroring-considerations.html) for traffic mirroring
///
/// ## Example Usage
///
/// To create a basic traffic mirror session
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let filter = traffic_mirror_filter::create(
///         "filter",
///         TrafficMirrorFilterArgs::builder()
///             .description("traffic mirror filter - example")
///             .network_services(vec!["amazon-dns",])
///             .build_struct(),
///     );
///     let session = traffic_mirror_session::create(
///         "session",
///         TrafficMirrorSessionArgs::builder()
///             .description("traffic mirror session - example")
///             .network_interface_id("${test.primaryNetworkInterfaceId}")
///             .session_number(1)
///             .traffic_mirror_filter_id("${filter.id}")
///             .traffic_mirror_target_id("${target.id}")
///             .build_struct(),
///     );
///     let target = traffic_mirror_target::create(
///         "target",
///         TrafficMirrorTargetArgs::builder()
///             .network_load_balancer_arn("${lb.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import traffic mirror sessions using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/trafficMirrorSession:TrafficMirrorSession session tms-0d8aa3ca35897b82e
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_mirror_session {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficMirrorSessionArgs {
        /// A description of the traffic mirror session.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the source network interface. Not all network interfaces are eligible as mirror sources. On EC2 instances only nitro based instances support mirroring.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do not specify this parameter when you want to mirror the entire packet. To mirror a subset of the packet, set this to the length (in bytes) that you want to mirror.
        #[builder(into, default)]
        pub packet_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.
        #[builder(into)]
        pub session_number: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the traffic mirror filter to be used
        #[builder(into)]
        pub traffic_mirror_filter_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the traffic mirror target to be used
        #[builder(into)]
        pub traffic_mirror_target_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN protocol, see RFC 7348. If you do not specify a VirtualNetworkId, an account-wide unique id is chosen at random.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TrafficMirrorSessionResult {
        /// The ARN of the traffic mirror session.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the traffic mirror session.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the source network interface. Not all network interfaces are eligible as mirror sources. On EC2 instances only nitro based instances support mirroring.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the session owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do not specify this parameter when you want to mirror the entire packet. To mirror a subset of the packet, set this to the length (in bytes) that you want to mirror.
        pub packet_length: pulumi_gestalt_rust::Output<i32>,
        /// The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.
        pub session_number: pulumi_gestalt_rust::Output<i32>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ID of the traffic mirror filter to be used
        pub traffic_mirror_filter_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the traffic mirror target to be used
        pub traffic_mirror_target_id: pulumi_gestalt_rust::Output<String>,
        /// The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN protocol, see RFC 7348. If you do not specify a VirtualNetworkId, an account-wide unique id is chosen at random.
        pub virtual_network_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficMirrorSessionArgs,
    ) -> TrafficMirrorSessionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let packet_length_binding = args.packet_length.get_output(context);
        let session_number_binding = args.session_number.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let traffic_mirror_filter_id_binding = args
            .traffic_mirror_filter_id
            .get_output(context);
        let traffic_mirror_target_id_binding = args
            .traffic_mirror_target_id
            .get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/trafficMirrorSession:TrafficMirrorSession".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: network_interface_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packetLength".into(),
                    value: packet_length_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionNumber".into(),
                    value: session_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficMirrorFilterId".into(),
                    value: traffic_mirror_filter_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficMirrorTargetId".into(),
                    value: traffic_mirror_target_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficMirrorSessionResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            network_interface_id: o.get_field("networkInterfaceId"),
            owner_id: o.get_field("ownerId"),
            packet_length: o.get_field("packetLength"),
            session_number: o.get_field("sessionNumber"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            traffic_mirror_filter_id: o.get_field("trafficMirrorFilterId"),
            traffic_mirror_target_id: o.get_field("trafficMirrorTargetId"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
