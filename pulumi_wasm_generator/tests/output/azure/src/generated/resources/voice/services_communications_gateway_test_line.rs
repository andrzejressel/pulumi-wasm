/// Manages a Voice Services Communications Gateway Test Line.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Central US
///   exampleServicesCommunicationsGateway:
///     type: azure:voice:ServicesCommunicationsGateway
///     name: example
///     properties:
///       name: example-vcg
///       resourceGroupName: ${example.name}
///   exampleServicesCommunicationsGatewayTestLine:
///     type: azure:voice:ServicesCommunicationsGatewayTestLine
///     name: example
///     properties:
///       name: example-vtl
///       location: West Central US
///       voiceServicesCommunicationsGatewayId: ${exampleServicesCommunicationsGateway.id}
///       phoneNumber: '123456789'
///       purpose: Automated
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Voice Services Communications Gateway Test Line can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.VoiceServices/communicationsGateways/communicationsGateway1/testLines/testLine1
/// ```
///
pub mod services_communications_gateway_test_line {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineArgs {
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the phone number.
        #[builder(into)]
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        #[builder(into)]
        pub purpose: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub voice_services_communications_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineResult {
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the phone number.
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        pub purpose: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        pub voice_services_communications_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let phone_number_binding = args.phone_number.get_inner();
        let purpose_binding = args.purpose.get_inner();
        let tags_binding = args.tags.get_inner();
        let voice_services_communications_gateway_id_binding = args
            .voice_services_communications_gateway_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "voiceServicesCommunicationsGatewayId".into(),
                    value: &voice_services_communications_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "purpose".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "voiceServicesCommunicationsGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServicesCommunicationsGatewayTestLineResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            purpose: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purpose").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            voice_services_communications_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceServicesCommunicationsGatewayId").unwrap(),
            ),
        }
    }
}
