/// Creates a device in a global network. If you specify both a site ID and a location,
/// the location of the site is used for visualization in the Network Manager console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceArgs {
        /// The AWS location of the device. Documented below.
        #[builder(into, default)]
        pub aws_location: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DeviceAwsLocation>,
        >,
        /// A description of the device.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the device. Documented below.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DeviceLocation>,
        >,
        /// The model of device.
        #[builder(into, default)]
        pub model: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The serial number of the device.
        #[builder(into, default)]
        pub serial_number: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the site.
        #[builder(into, default)]
        pub site_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of device.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The vendor of the device.
        #[builder(into, default)]
        pub vendor: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeviceResult {
        /// The Amazon Resource Name (ARN) of the device.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS location of the device. Documented below.
        pub aws_location: pulumi_wasm_rust::Output<
            Option<super::super::types::networkmanager::DeviceAwsLocation>,
        >,
        /// A description of the device.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the global network.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The location of the device. Documented below.
        pub location: pulumi_wasm_rust::Output<
            Option<super::super::types::networkmanager::DeviceLocation>,
        >,
        /// The model of device.
        pub model: pulumi_wasm_rust::Output<Option<String>>,
        /// The serial number of the device.
        pub serial_number: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the site.
        pub site_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of device.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The vendor of the device.
        pub vendor: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DeviceArgs,
    ) -> DeviceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsLocation".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "model".into(),
                },
                register_interface::ResultField {
                    name: "serialNumber".into(),
                },
                register_interface::ResultField {
                    name: "siteId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vendor".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeviceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsLocation").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("model").unwrap(),
            ),
            serial_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialNumber").unwrap(),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vendor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vendor").unwrap(),
            ),
        }
    }
}
