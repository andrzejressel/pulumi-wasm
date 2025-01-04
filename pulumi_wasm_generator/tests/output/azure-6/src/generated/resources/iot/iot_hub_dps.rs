/// Manages an IotHub Device Provisioning Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIotHubDps:
///     type: azure:iot:IotHubDps
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationPolicy: Hashed
///       sku:
///         name: S1
///         capacity: '1'
/// ```
///
/// ## Import
///
/// IoT Device Provisioning Service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubDps:IotHubDps example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/provisioningServices/example
/// ```
///
pub mod iot_hub_dps {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDpsArgs {
        /// The allocation policy of the IoT Device Provisioning Service (`Hashed`, `GeoLatency` or `Static`). Defaults to `Hashed`.
        #[builder(into, default)]
        pub allocation_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if the IoT Device Provisioning Service has data residency and disaster recovery enabled. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_residency_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `ip_filter_rule` block as defined below.
        #[builder(into, default)]
        pub ip_filter_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsIpFilterRule>>,
        >,
        /// A `linked_hub` block as defined below.
        #[builder(into, default)]
        pub linked_hubs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsLinkedHub>>,
        >,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Iot Device Provisioning Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether requests from Public Network are allowed. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the Iot Device Provisioning Service resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<super::super::types::iot::IotHubDpsSku>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IotHubDpsResult {
        /// The allocation policy of the IoT Device Provisioning Service (`Hashed`, `GeoLatency` or `Static`). Defaults to `Hashed`.
        pub allocation_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if the IoT Device Provisioning Service has data residency and disaster recovery enabled. Defaults to `false`. Changing this forces a new resource to be created.
        pub data_residency_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The device endpoint of the IoT Device Provisioning Service.
        pub device_provisioning_host_name: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the IoT Device Provisioning Service.
        pub id_scope: pulumi_wasm_rust::Output<String>,
        /// An `ip_filter_rule` block as defined below.
        pub ip_filter_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsIpFilterRule>>,
        >,
        /// A `linked_hub` block as defined below.
        pub linked_hubs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsLinkedHub>>,
        >,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Iot Device Provisioning Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether requests from Public Network are allowed. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the Iot Device Provisioning Service resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The service endpoint of the IoT Device Provisioning Service.
        pub service_operations_host_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<super::super::types::iot::IotHubDpsSku>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IotHubDpsArgs) -> IotHubDpsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_policy_binding = args.allocation_policy.get_inner();
        let data_residency_enabled_binding = args.data_residency_enabled.get_inner();
        let ip_filter_rules_binding = args.ip_filter_rules.get_inner();
        let linked_hubs_binding = args.linked_hubs.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/iotHubDps:IotHubDps".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationPolicy".into(),
                    value: &allocation_policy_binding,
                },
                register_interface::ObjectField {
                    name: "dataResidencyEnabled".into(),
                    value: &data_residency_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ipFilterRules".into(),
                    value: &ip_filter_rules_binding,
                },
                register_interface::ObjectField {
                    name: "linkedHubs".into(),
                    value: &linked_hubs_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocationPolicy".into(),
                },
                register_interface::ResultField {
                    name: "dataResidencyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "deviceProvisioningHostName".into(),
                },
                register_interface::ResultField {
                    name: "idScope".into(),
                },
                register_interface::ResultField {
                    name: "ipFilterRules".into(),
                },
                register_interface::ResultField {
                    name: "linkedHubs".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceOperationsHostName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
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
        IotHubDpsResult {
            allocation_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationPolicy").unwrap(),
            ),
            data_residency_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataResidencyEnabled").unwrap(),
            ),
            device_provisioning_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceProvisioningHostName").unwrap(),
            ),
            id_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idScope").unwrap(),
            ),
            ip_filter_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipFilterRules").unwrap(),
            ),
            linked_hubs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedHubs").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_operations_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceOperationsHostName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
