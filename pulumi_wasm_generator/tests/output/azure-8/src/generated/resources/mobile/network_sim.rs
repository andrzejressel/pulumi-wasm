/// Manages a Mobile Network Sim.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleNetwork = network::create(
///         "exampleNetwork",
///         NetworkArgs::builder()
///             .location("${example.location}")
///             .mobile_country_code("001")
///             .mobile_network_code("01")
///             .name("example-mn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkAttachedDataNetwork = network_attached_data_network::create(
///         "exampleNetworkAttachedDataNetwork",
///         NetworkAttachedDataNetworkArgs::builder()
///             .dns_addresses(vec!["1.1.1.1",])
///             .location("${example.location}")
///             .mobile_network_data_network_name(
///                 "${exampleAzurermMobileNetworkDataNetwork.name}",
///             )
///             .mobile_network_packet_core_data_plane_id(
///                 "${exampleAzurermMobileNetworkPacketCoreDataPlane.id}",
///             )
///             .user_equipment_address_pool_prefixes(vec!["2.4.0.0/24",])
///             .user_equipment_static_address_pool_prefixes(vec!["2.4.1.0/24",])
///             .user_plane_access_ipv_4_address("10.204.141.4")
///             .user_plane_access_ipv_4_gateway("10.204.141.1")
///             .user_plane_access_ipv_4_subnet("10.204.141.0/24")
///             .user_plane_access_name("test")
///             .build_struct(),
///     );
///     let exampleNetworkSim = network_sim::create(
///         "exampleNetworkSim",
///         NetworkSimArgs::builder()
///             .authentication_key("00000000000000000000000000000000")
///             .integrated_circuit_card_identifier("8900000000000000000")
///             .international_mobile_subscriber_identity("000000000000000")
///             .mobile_network_sim_group_id("${exampleNetworkSimGroup.id}")
///             .name("example-sim")
///             .operator_key_code("00000000000000000000000000000000")
///             .static_ip_configurations(
///                 vec![
///                     NetworkSimStaticIpConfiguration::builder()
///                     .attachedDataNetworkId("${test.id}")
///                     .sliceId("${testAzurermMobileNetworkSlice.id}")
///                     .staticIpv4Address("2.4.0.1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleNetworkSimGroup = network_sim_group::create(
///         "exampleNetworkSimGroup",
///         NetworkSimGroupArgs::builder()
///             .location("${example.location}")
///             .mobile_network_id("${exampleNetwork.id}")
///             .name("example-mnsg")
///             .build_struct(),
///     );
///     let exampleNetworkSlice = network_slice::create(
///         "exampleNetworkSlice",
///         NetworkSliceArgs::builder()
///             .location("${example.location}")
///             .mobile_network_id("${exampleNetwork.id}")
///             .name("example-slice")
///             .single_network_slice_selection_assistance_information(
///                 NetworkSliceSingleNetworkSliceSelectionAssistanceInformation::builder()
///                     .sliceServiceType(1)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Mobile Network Sim can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSim:NetworkSim example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/simGroups/simGroup1/sims/sim1
/// ```
///
pub mod network_sim {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimArgs {
        /// The Ki value for the SIM.
        #[builder(into)]
        pub authentication_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// An optional free-form text field that can be used to record the device type this SIM is associated with, for example `Video camera`. The Azure portal allows SIMs to be grouped and filtered based on this value.
        #[builder(into, default)]
        pub device_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The integrated circuit card ID (ICCID) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub integrated_circuit_card_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The international mobile subscriber identity (IMSI) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub international_mobile_subscriber_identity: pulumi_wasm_rust::InputOrOutput<
            String,
        >,
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub mobile_network_sim_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Opc value for the SIM.
        #[builder(into)]
        pub operator_key_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of SIM policy used by this SIM.
        #[builder(into, default)]
        pub sim_policy_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `static_ip_configuration` block as defined below.
        #[builder(into, default)]
        pub static_ip_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::mobile::NetworkSimStaticIpConfiguration>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimResult {
        /// The Ki value for the SIM.
        pub authentication_key: pulumi_wasm_rust::Output<String>,
        /// An optional free-form text field that can be used to record the device type this SIM is associated with, for example `Video camera`. The Azure portal allows SIMs to be grouped and filtered based on this value.
        pub device_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The integrated circuit card ID (ICCID) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        pub integrated_circuit_card_identifier: pulumi_wasm_rust::Output<String>,
        /// The international mobile subscriber identity (IMSI) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        pub international_mobile_subscriber_identity: pulumi_wasm_rust::Output<String>,
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to. Changing this forces a new Mobile Network Sim to be created.
        pub mobile_network_sim_group_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim. Changing this forces a new Mobile Network Sim to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Opc value for the SIM.
        pub operator_key_code: pulumi_wasm_rust::Output<String>,
        /// The ID of SIM policy used by this SIM.
        pub sim_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of the SIM resource.
        pub sim_state: pulumi_wasm_rust::Output<String>,
        /// A `static_ip_configuration` block as defined below.
        pub static_ip_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::mobile::NetworkSimStaticIpConfiguration>>,
        >,
        /// The public key fingerprint of the SIM vendor who provided this SIM, if any.
        pub vendor_key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// The name of the SIM vendor who provided this SIM, if any.
        pub vendor_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkSimArgs,
    ) -> NetworkSimResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_key_binding = args
            .authentication_key
            .get_output(context)
            .get_inner();
        let device_type_binding = args.device_type.get_output(context).get_inner();
        let integrated_circuit_card_identifier_binding = args
            .integrated_circuit_card_identifier
            .get_output(context)
            .get_inner();
        let international_mobile_subscriber_identity_binding = args
            .international_mobile_subscriber_identity
            .get_output(context)
            .get_inner();
        let mobile_network_sim_group_id_binding = args
            .mobile_network_sim_group_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let operator_key_code_binding = args
            .operator_key_code
            .get_output(context)
            .get_inner();
        let sim_policy_id_binding = args.sim_policy_id.get_output(context).get_inner();
        let static_ip_configurations_binding = args
            .static_ip_configurations
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkSim:NetworkSim".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationKey".into(),
                    value: &authentication_key_binding,
                },
                register_interface::ObjectField {
                    name: "deviceType".into(),
                    value: &device_type_binding,
                },
                register_interface::ObjectField {
                    name: "integratedCircuitCardIdentifier".into(),
                    value: &integrated_circuit_card_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "internationalMobileSubscriberIdentity".into(),
                    value: &international_mobile_subscriber_identity_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkSimGroupId".into(),
                    value: &mobile_network_sim_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "operatorKeyCode".into(),
                    value: &operator_key_code_binding,
                },
                register_interface::ObjectField {
                    name: "simPolicyId".into(),
                    value: &sim_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "staticIpConfigurations".into(),
                    value: &static_ip_configurations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationKey".into(),
                },
                register_interface::ResultField {
                    name: "deviceType".into(),
                },
                register_interface::ResultField {
                    name: "integratedCircuitCardIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "internationalMobileSubscriberIdentity".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkSimGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "operatorKeyCode".into(),
                },
                register_interface::ResultField {
                    name: "simPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "simState".into(),
                },
                register_interface::ResultField {
                    name: "staticIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "vendorKeyFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "vendorName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkSimResult {
            authentication_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationKey").unwrap(),
            ),
            device_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceType").unwrap(),
            ),
            integrated_circuit_card_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integratedCircuitCardIdentifier").unwrap(),
            ),
            international_mobile_subscriber_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internationalMobileSubscriberIdentity").unwrap(),
            ),
            mobile_network_sim_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkSimGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            operator_key_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operatorKeyCode").unwrap(),
            ),
            sim_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("simPolicyId").unwrap(),
            ),
            sim_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("simState").unwrap(),
            ),
            static_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticIpConfigurations").unwrap(),
            ),
            vendor_key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vendorKeyFingerprint").unwrap(),
            ),
            vendor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vendorName").unwrap(),
            ),
        }
    }
}
