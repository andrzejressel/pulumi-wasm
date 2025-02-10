/// Associates a link to a device.
/// A device can be associated to multiple links and a link can be associated to multiple devices.
/// The device and link must be in the same global network and the same site.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = link_association::create(
///         "example",
///         LinkAssociationArgs::builder()
///             .device_id("${exampleAwsNetworkmanagerDevice.id}")
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .link_id("${exampleAwsNetworkmanagerLink.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_link_association` using the global network ID, link ID and device ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/linkAssociation:LinkAssociation example global-network-0d47f6t230mz46dy4,link-444555aaabbb11223,device-07f6fd08867abc123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod link_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkAssociationArgs {
        /// The ID of the device.
        #[builder(into)]
        pub device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the link.
        #[builder(into)]
        pub link_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkAssociationResult {
        /// The ID of the device.
        pub device_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the link.
        pub link_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkAssociationArgs,
    ) -> LinkAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_id_binding = args.device_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let link_id_binding = args.link_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/linkAssociation:LinkAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
            ],
        };
        let o = context.register_resource(request);
        LinkAssociationResult {
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            link_id: o.get_field("linkId"),
        }
    }
}
