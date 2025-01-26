pub mod get_network_sim {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSimArgs {
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to.
        #[builder(into)]
        pub mobile_network_sim_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSimResult {
        /// The device type this SIM is associated with.
        pub device_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The integrated circuit card ID (ICCID) for the SIM.
        pub integrated_circuit_card_identifier: pulumi_wasm_rust::Output<String>,
        /// The international mobile subscriber identity (IMSI) for the SIM.
        pub international_mobile_subscriber_identity: pulumi_wasm_rust::Output<String>,
        pub mobile_network_sim_group_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of SIM policy used by this SIM.
        pub sim_policy_id: pulumi_wasm_rust::Output<String>,
        /// The state of the SIM resource.
        pub sim_state: pulumi_wasm_rust::Output<String>,
        /// A `static_ip_configuration` block as defined below.
        pub static_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkSimStaticIpConfiguration>,
        >,
        /// The public key fingerprint of the SIM vendor who provided this SIM.
        pub vendor_key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// The name of the SIM vendor who provided this SIM, if any.
        pub vendor_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkSimArgs,
    ) -> GetNetworkSimResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mobile_network_sim_group_id_binding = args
            .mobile_network_sim_group_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkSim:getNetworkSim".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkSimGroupId".into(),
                    value: &mobile_network_sim_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deviceType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkSimResult {
            device_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
