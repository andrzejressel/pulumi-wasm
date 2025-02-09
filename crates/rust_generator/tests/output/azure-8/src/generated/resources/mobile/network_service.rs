/// Manages a Mobile Network Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: east us
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkService:
///     type: azure:mobile:NetworkService
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       servicePrecedence: 0
///       pccRules:
///         - name: default-rule
///           precedence: 1
///           trafficControlEnabled: true
///           qosPolicy:
///             allocationAndRetentionPriorityLevel: 9
///             qosIndicator: 9
///             preemptionCapability: NotPreempt
///             preemptionVulnerability: Preemptable
///             guaranteedBitRate:
///               downlink: 100 Mbps
///               uplink: 10 Mbps
///             maximumBitRate:
///               downlink: 1 Gbps
///               uplink: 100 Mbps
///           serviceDataFlowTemplates:
///             - direction: Uplink
///               name: IP-to-server
///               ports: []
///               protocols:
///                 - ip
///               remoteIpLists:
///                 - 10.3.4.0/24
///       serviceQosPolicy:
///         allocationAndRetentionPriorityLevel: 9
///         qosIndicator: 9
///         preemptionCapability: NotPreempt
///         preemptionVulnerability: Preemptable
///         maximumBitRate:
///           downlink: 1 Gbps
///           uplink: 100 Mbps
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkService:NetworkService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1/services/service1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkServiceArgs {
        /// Specifies the Azure Region where the Mobile Network Service should exist. Changing this forces a new Mobile Network Service to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        #[builder(into)]
        pub pcc_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::mobile::NetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all services configured in the mobile network. Must be between `0` and `255`.
        #[builder(into)]
        pub service_precedence: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service. This can be overridden for particular flows using the ruleQosPolicy field in a `pcc_rule`. If this field is not specified then the `sim_policy` of User Equipment (UE) will define the QoS settings.
        #[builder(into, default)]
        pub service_qos_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mobile::NetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkServiceResult {
        /// Specifies the Azure Region where the Mobile Network Service should exist. Changing this forces a new Mobile Network Service to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        pub pcc_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mobile::NetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all services configured in the mobile network. Must be between `0` and `255`.
        pub service_precedence: pulumi_gestalt_rust::Output<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service. This can be overridden for particular flows using the ruleQosPolicy field in a `pcc_rule`. If this field is not specified then the `sim_policy` of User Equipment (UE) will define the QoS settings.
        pub service_qos_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::mobile::NetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkServiceArgs,
    ) -> NetworkServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let mobile_network_id_binding = args.mobile_network_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let pcc_rules_binding = args.pcc_rules.get_output(context);
        let service_precedence_binding = args.service_precedence.get_output(context);
        let service_qos_policy_binding = args.service_qos_policy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkService:NetworkService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: mobile_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pccRules".into(),
                    value: pcc_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrecedence".into(),
                    value: service_precedence_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceQosPolicy".into(),
                    value: service_qos_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkServiceResult {
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            pcc_rules: o.get_field("pccRules"),
            service_precedence: o.get_field("servicePrecedence"),
            service_qos_policy: o.get_field("serviceQosPolicy"),
            tags: o.get_field("tags"),
        }
    }
}
