/// Creates a connection between two devices.
/// The devices can be a physical or virtual appliance that connects to a third-party appliance in a VPC, or a physical appliance that connects to another physical appliance in an on-premises network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .connected_device_id("${example2.id}")
///             .device_id("${example1.id}")
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_connection` using the connection ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/connection:Connection example arn:aws:networkmanager::123456789012:device/global-network-0d47f6t230mz46dy4/connection-07f6fd08867abc123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The ID of the second device in the connection.
        #[builder(into)]
        pub connected_device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the link for the second device.
        #[builder(into, default)]
        pub connected_link_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of the connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the first device in the connection.
        #[builder(into)]
        pub device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the link for the first device.
        #[builder(into, default)]
        pub link_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The Amazon Resource Name (ARN) of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the second device in the connection.
        pub connected_device_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the link for the second device.
        pub connected_link_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of the connection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the first device in the connection.
        pub device_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the link for the first device.
        pub link_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connected_device_id_binding = args.connected_device_id.get_output(context);
        let connected_link_id_binding = args.connected_link_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let device_id_binding = args.device_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let link_id_binding = args.link_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectedDeviceId".into(),
                    value: connected_device_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectedLinkId".into(),
                    value: connected_link_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceId".into(),
                    value: device_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkId".into(),
                    value: link_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            arn: o.get_field("arn"),
            connected_device_id: o.get_field("connectedDeviceId"),
            connected_link_id: o.get_field("connectedLinkId"),
            description: o.get_field("description"),
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            link_id: o.get_field("linkId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
