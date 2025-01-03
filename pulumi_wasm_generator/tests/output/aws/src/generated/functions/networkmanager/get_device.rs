pub mod get_device {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDeviceArgs {
        /// ID of the device.
        #[builder(into)]
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the device.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDeviceResult {
        /// ARN of the device.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS location of the device. Documented below.
        pub aws_locations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::networkmanager::GetDeviceAwsLocation>,
        >,
        /// Description of the device.
        pub description: pulumi_wasm_rust::Output<String>,
        pub device_id: pulumi_wasm_rust::Output<String>,
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Location of the device. Documented below.
        pub locations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::networkmanager::GetDeviceLocation>,
        >,
        /// Model of device.
        pub model: pulumi_wasm_rust::Output<String>,
        /// Serial number of the device.
        pub serial_number: pulumi_wasm_rust::Output<String>,
        /// ID of the site.
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the device.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of device.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Vendor of the device.
        pub vendor: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDeviceArgs) -> GetDeviceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_id_binding = args.device_id.get_inner();
        let global_network_id_binding = args.global_network_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkmanager/getDevice:getDevice".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsLocations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "deviceId".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "locations".into(),
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
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vendor".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDeviceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsLocations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            device_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceId").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locations").unwrap(),
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
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vendor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vendor").unwrap(),
            ),
        }
    }
}
