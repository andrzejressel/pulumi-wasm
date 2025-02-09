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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iot_hub_dps {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDpsArgs {
        /// The allocation policy of the IoT Device Provisioning Service (`Hashed`, `GeoLatency` or `Static`). Defaults to `Hashed`.
        #[builder(into, default)]
        pub allocation_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the IoT Device Provisioning Service has data residency and disaster recovery enabled. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_residency_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `ip_filter_rule` block as defined below.
        #[builder(into, default)]
        pub ip_filter_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IotHubDpsIpFilterRule>>,
        >,
        /// A `linked_hub` block as defined below.
        #[builder(into, default)]
        pub linked_hubs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IotHubDpsLinkedHub>>,
        >,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Iot Device Provisioning Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether requests from Public Network are allowed. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group under which the Iot Device Provisioning Service resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::iot::IotHubDpsSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IotHubDpsResult {
        /// The allocation policy of the IoT Device Provisioning Service (`Hashed`, `GeoLatency` or `Static`). Defaults to `Hashed`.
        pub allocation_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the IoT Device Provisioning Service has data residency and disaster recovery enabled. Defaults to `false`. Changing this forces a new resource to be created.
        pub data_residency_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The device endpoint of the IoT Device Provisioning Service.
        pub device_provisioning_host_name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the IoT Device Provisioning Service.
        pub id_scope: pulumi_gestalt_rust::Output<String>,
        /// An `ip_filter_rule` block as defined below.
        pub ip_filter_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsIpFilterRule>>,
        >,
        /// A `linked_hub` block as defined below.
        pub linked_hubs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::iot::IotHubDpsLinkedHub>>,
        >,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Iot Device Provisioning Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether requests from Public Network are allowed. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group under which the Iot Device Provisioning Service resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The service endpoint of the IoT Device Provisioning Service.
        pub service_operations_host_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::iot::IotHubDpsSku>,
        /// A mapping of tags to assign to the resource.
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
        args: IotHubDpsArgs,
    ) -> IotHubDpsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocation_policy_binding = args.allocation_policy.get_output(context);
        let data_residency_enabled_binding = args
            .data_residency_enabled
            .get_output(context);
        let ip_filter_rules_binding = args.ip_filter_rules.get_output(context);
        let linked_hubs_binding = args.linked_hubs.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/iotHubDps:IotHubDps".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocationPolicy".into(),
                    value: allocation_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataResidencyEnabled".into(),
                    value: data_residency_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipFilterRules".into(),
                    value: ip_filter_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedHubs".into(),
                    value: linked_hubs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IotHubDpsResult {
            allocation_policy: o.get_field("allocationPolicy"),
            data_residency_enabled: o.get_field("dataResidencyEnabled"),
            device_provisioning_host_name: o.get_field("deviceProvisioningHostName"),
            id_scope: o.get_field("idScope"),
            ip_filter_rules: o.get_field("ipFilterRules"),
            linked_hubs: o.get_field("linkedHubs"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_operations_host_name: o.get_field("serviceOperationsHostName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
