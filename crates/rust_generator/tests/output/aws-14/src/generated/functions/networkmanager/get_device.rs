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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDeviceArgs,
    ) -> GetDeviceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_id_binding = args.device_id.get_output(context).get_inner();
        let global_network_id_binding = args
            .global_network_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkmanager/getDevice:getDevice".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceId".into(),
                    value: &device_id_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDeviceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsLocations"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            device_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceId"),
            ),
            global_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalNetworkId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locations"),
            ),
            model: pulumi_gestalt_rust::__private::into_domain(o.extract_field("model")),
            serial_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialNumber"),
            ),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vendor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vendor"),
            ),
        }
    }
}
