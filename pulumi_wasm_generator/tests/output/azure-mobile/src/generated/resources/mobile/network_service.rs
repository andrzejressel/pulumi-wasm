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
pub mod network_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkServiceArgs {
        /// Specifies the Azure Region where the Mobile Network Service should exist. Changing this forces a new Mobile Network Service to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        #[builder(into)]
        pub pcc_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::mobile::NetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all services configured in the mobile network. Must be between `0` and `255`.
        #[builder(into)]
        pub service_precedence: pulumi_wasm_rust::Output<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service. This can be overridden for particular flows using the ruleQosPolicy field in a `pcc_rule`. If this field is not specified then the `sim_policy` of User Equipment (UE) will define the QoS settings.
        #[builder(into, default)]
        pub service_qos_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::mobile::NetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkServiceResult {
        /// Specifies the Azure Region where the Mobile Network Service should exist. Changing this forces a new Mobile Network Service to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Service. Changing this forces a new Mobile Network Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        pub pcc_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::mobile::NetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all services configured in the mobile network. Must be between `0` and `255`.
        pub service_precedence: pulumi_wasm_rust::Output<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service. This can be overridden for particular flows using the ruleQosPolicy field in a `pcc_rule`. If this field is not specified then the `sim_policy` of User Equipment (UE) will define the QoS settings.
        pub service_qos_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::mobile::NetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkServiceArgs) -> NetworkServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let pcc_rules_binding = args.pcc_rules.get_inner();
        let service_precedence_binding = args.service_precedence.get_inner();
        let service_qos_policy_binding = args.service_qos_policy.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkService:NetworkService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pccRules".into(),
                    value: &pcc_rules_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrecedence".into(),
                    value: &service_precedence_binding,
                },
                register_interface::ObjectField {
                    name: "serviceQosPolicy".into(),
                    value: &service_qos_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pccRules".into(),
                },
                register_interface::ResultField {
                    name: "servicePrecedence".into(),
                },
                register_interface::ResultField {
                    name: "serviceQosPolicy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkServiceResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pcc_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pccRules").unwrap(),
            ),
            service_precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrecedence").unwrap(),
            ),
            service_qos_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceQosPolicy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
