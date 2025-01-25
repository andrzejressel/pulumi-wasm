/// Associates a link to a device.
/// A device can be associated to multiple links and a link can be associated to multiple devices.
/// The device and link must be in the same global network and the same site.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod link_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkAssociationArgs {
        /// The ID of the device.
        #[builder(into)]
        pub device_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the link.
        #[builder(into)]
        pub link_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkAssociationResult {
        /// The ID of the device.
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link.
        pub link_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LinkAssociationArgs,
    ) -> LinkAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_id_binding = args.device_id.get_output(context).get_inner();
        let global_network_id_binding = args
            .global_network_id
            .get_output(context)
            .get_inner();
        let link_id_binding = args.link_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/linkAssociation:LinkAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "linkId".into(),
                    value: &link_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deviceId".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkAssociationResult {
            device_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceId").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
        }
    }
}
