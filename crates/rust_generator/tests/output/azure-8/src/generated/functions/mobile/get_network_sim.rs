#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_sim {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSimArgs {
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to.
        #[builder(into)]
        pub mobile_network_sim_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSimResult {
        /// The device type this SIM is associated with.
        pub device_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The integrated circuit card ID (ICCID) for the SIM.
        pub integrated_circuit_card_identifier: pulumi_gestalt_rust::Output<String>,
        /// The international mobile subscriber identity (IMSI) for the SIM.
        pub international_mobile_subscriber_identity: pulumi_gestalt_rust::Output<
            String,
        >,
        pub mobile_network_sim_group_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of SIM policy used by this SIM.
        pub sim_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The state of the SIM resource.
        pub sim_state: pulumi_gestalt_rust::Output<String>,
        /// A `static_ip_configuration` block as defined below.
        pub static_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkSimStaticIpConfiguration>,
        >,
        /// The public key fingerprint of the SIM vendor who provided this SIM.
        pub vendor_key_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name of the SIM vendor who provided this SIM, if any.
        pub vendor_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkSimArgs,
    ) -> GetNetworkSimResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_sim_group_id_binding = args
            .mobile_network_sim_group_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkSim:getNetworkSim".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkSimGroupId".into(),
                    value: mobile_network_sim_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkSimResult {
            device_type: o.get_field("deviceType"),
            id: o.get_field("id"),
            integrated_circuit_card_identifier: o
                .get_field("integratedCircuitCardIdentifier"),
            international_mobile_subscriber_identity: o
                .get_field("internationalMobileSubscriberIdentity"),
            mobile_network_sim_group_id: o.get_field("mobileNetworkSimGroupId"),
            name: o.get_field("name"),
            sim_policy_id: o.get_field("simPolicyId"),
            sim_state: o.get_field("simState"),
            static_ip_configurations: o.get_field("staticIpConfigurations"),
            vendor_key_fingerprint: o.get_field("vendorKeyFingerprint"),
            vendor_name: o.get_field("vendorName"),
        }
    }
}
