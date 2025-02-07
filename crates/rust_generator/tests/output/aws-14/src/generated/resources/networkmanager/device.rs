/// Creates a device in a global network. If you specify both a site ID and a location,
/// the location of the site is used for visualization in the Network Manager console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device::create(
///         "example",
///         DeviceArgs::builder()
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .site_id("${exampleAwsNetworkmanagerSite.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_device` using the device ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/device:Device example arn:aws:networkmanager::123456789012:device/global-network-0d47f6t230mz46dy4/device-07f6fd08867abc123
/// ```
pub mod device {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceArgs {
        /// The AWS location of the device. Documented below.
        #[builder(into, default)]
        pub aws_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DeviceAwsLocation>,
        >,
        /// A description of the device.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the device. Documented below.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DeviceLocation>,
        >,
        /// The model of device.
        #[builder(into, default)]
        pub model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The serial number of the device.
        #[builder(into, default)]
        pub serial_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the site.
        #[builder(into, default)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of device.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The vendor of the device.
        #[builder(into, default)]
        pub vendor: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeviceResult {
        /// The Amazon Resource Name (ARN) of the device.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS location of the device. Documented below.
        pub aws_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::DeviceAwsLocation>,
        >,
        /// A description of the device.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the global network.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the device. Documented below.
        pub location: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::DeviceLocation>,
        >,
        /// The model of device.
        pub model: pulumi_gestalt_rust::Output<Option<String>>,
        /// The serial number of the device.
        pub serial_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the site.
        pub site_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of device.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The vendor of the device.
        pub vendor: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeviceArgs,
    ) -> DeviceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aws_location_binding = args.aws_location.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let global_network_id_binding = args
            .global_network_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let model_binding = args.model.get_output(context).get_inner();
        let serial_number_binding = args.serial_number.get_output(context).get_inner();
        let site_id_binding = args.site_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let vendor_binding = args.vendor.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/device:Device".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsLocation".into(),
                    value: &aws_location_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "model".into(),
                    value: &model_binding,
                },
                register_interface::ObjectField {
                    name: "serialNumber".into(),
                    value: &serial_number_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vendor".into(),
                    value: &vendor_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeviceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsLocation"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            global_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalNetworkId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            model: pulumi_gestalt_rust::__private::into_domain(o.extract_field("model")),
            serial_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialNumber"),
            ),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vendor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vendor"),
            ),
        }
    }
}
