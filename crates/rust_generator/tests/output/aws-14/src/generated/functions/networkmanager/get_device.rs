#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_device {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDeviceArgs {
        /// ID of the device.
        #[builder(into)]
        pub device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the device.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDeviceResult {
        /// ARN of the device.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS location of the device. Documented below.
        pub aws_locations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkmanager::GetDeviceAwsLocation>,
        >,
        /// Description of the device.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub device_id: pulumi_gestalt_rust::Output<String>,
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Location of the device. Documented below.
        pub locations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkmanager::GetDeviceLocation>,
        >,
        /// Model of device.
        pub model: pulumi_gestalt_rust::Output<String>,
        /// Serial number of the device.
        pub serial_number: pulumi_gestalt_rust::Output<String>,
        /// ID of the site.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the device.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of device.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Vendor of the device.
        pub vendor: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDeviceArgs,
    ) -> GetDeviceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_id_binding = args.device_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getDevice:getDevice".into(),
            version: super::super::super::get_version(),
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
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDeviceResult {
            arn: o.get_field("arn"),
            aws_locations: o.get_field("awsLocations"),
            description: o.get_field("description"),
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            id: o.get_field("id"),
            locations: o.get_field("locations"),
            model: o.get_field("model"),
            serial_number: o.get_field("serialNumber"),
            site_id: o.get_field("siteId"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            vendor: o.get_field("vendor"),
        }
    }
}
