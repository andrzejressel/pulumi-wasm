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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_device_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityDeviceGroupArgs {
        /// an `allow_rule` blocks as defined below.
        #[builder(into, default)]
        pub allow_rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::SecurityDeviceGroupAllowRule>,
        >,
        /// The ID of the IoT Hub which to link the Security Device Group to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Device Security Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `range_rule` blocks as defined below.
        #[builder(into, default)]
        pub range_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::SecurityDeviceGroupRangeRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecurityDeviceGroupResult {
        /// an `allow_rule` blocks as defined below.
        pub allow_rule: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::SecurityDeviceGroupAllowRule>,
        >,
        /// The ID of the IoT Hub which to link the Security Device Group to. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Device Security Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `range_rule` blocks as defined below.
        pub range_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::iot::SecurityDeviceGroupRangeRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityDeviceGroupArgs,
    ) -> SecurityDeviceGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_rule_binding = args.allow_rule.get_output(context);
        let iothub_id_binding = args.iothub_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let range_rules_binding = args.range_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/securityDeviceGroup:SecurityDeviceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowRule".into(),
                    value: allow_rule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubId".into(),
                    value: iothub_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rangeRules".into(),
                    value: range_rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityDeviceGroupResult {
            allow_rule: o.get_field("allowRule"),
            iothub_id: o.get_field("iothubId"),
            name: o.get_field("name"),
            range_rules: o.get_field("rangeRules"),
        }
    }
}
