/// Manages a Iot Security Device Group.
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
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example-IoTHub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleSecuritySolution:
///     type: azure:iot:SecuritySolution
///     name: example
///     properties:
///       name: example-Iot-Security-Solution
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       displayName: Iot Security Solution
///       iothubIds:
///         - ${exampleIoTHub.id}
///   exampleSecurityDeviceGroup:
///     type: azure:iot:SecurityDeviceGroup
///     name: example
///     properties:
///       name: example-device-security-group
///       iothubId: ${exampleIoTHub.id}
///       allowRule:
///         connectionToIpsNotAlloweds:
///           - 10.0.0.0/24
///       rangeRules:
///         - type: ActiveConnectionsNotInAllowedRange
///           min: 0
///           max: 30
///           duration: PT5M
///     options:
///       dependsOn:
///         - ${exampleSecuritySolution}
/// ```
///
/// ## Import
///
/// Iot Security Device Group can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/securityDeviceGroup:SecurityDeviceGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Devices/iotHubs/hub1/providers/Microsoft.Security/deviceSecurityGroups/group1
/// ```
///
pub mod security_device_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityDeviceGroupArgs {
        /// an `allow_rule` blocks as defined below.
        #[builder(into, default)]
        pub allow_rule: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::SecurityDeviceGroupAllowRule>,
        >,
        /// The ID of the IoT Hub which to link the Security Device Group to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Device Security Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `range_rule` blocks as defined below.
        #[builder(into, default)]
        pub range_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::SecurityDeviceGroupRangeRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecurityDeviceGroupResult {
        /// an `allow_rule` blocks as defined below.
        pub allow_rule: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::SecurityDeviceGroupAllowRule>,
        >,
        /// The ID of the IoT Hub which to link the Security Device Group to. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Device Security Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `range_rule` blocks as defined below.
        pub range_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::SecurityDeviceGroupRangeRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SecurityDeviceGroupArgs,
    ) -> SecurityDeviceGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_rule_binding = args.allow_rule.get_inner();
        let iothub_id_binding = args.iothub_id.get_inner();
        let name_binding = args.name.get_inner();
        let range_rules_binding = args.range_rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/securityDeviceGroup:SecurityDeviceGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowRule".into(),
                    value: &allow_rule_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rangeRules".into(),
                    value: &range_rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowRule".into(),
                },
                register_interface::ResultField {
                    name: "iothubId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rangeRules".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityDeviceGroupResult {
            allow_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowRule").unwrap(),
            ),
            iothub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            range_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangeRules").unwrap(),
            ),
        }
    }
}