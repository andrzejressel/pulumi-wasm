/// Promotes an App Service Slot to Production within an App Service.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.WebAppActiveSlot` resource instead.
///
/// > **Note:** When using Slots - the `app_settings`, `connection_string` and `site_config` blocks on the `azure.appservice.AppService` resource will be overwritten when promoting a Slot using the `azure.appservice.ActiveSlot` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///   example:
///     type: azure:core:ResourceGroup
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///   exampleSlot:
///     type: azure:appservice:Slot
///     name: example
///   exampleActiveSlot:
///     type: azure:appservice:ActiveSlot
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       appServiceName: ${exampleAppService.name}
///       appServiceSlotName: ${exampleSlot.name}
/// ```
pub mod active_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveSlotArgs {
        /// The name of the App Service within which the Slot exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_wasm_rust::Output<String>,
        /// The name of the App Service Slot which should be promoted to the Production Slot within the App Service.
        #[builder(into)]
        pub app_service_slot_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ActiveSlotResult {
        /// The name of the App Service within which the Slot exists. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_wasm_rust::Output<String>,
        /// The name of the App Service Slot which should be promoted to the Production Slot within the App Service.
        pub app_service_slot_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ActiveSlotArgs) -> ActiveSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_name_binding = args.app_service_name.get_inner();
        let app_service_slot_name_binding = args.app_service_slot_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/activeSlot:ActiveSlot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding,
                },
                register_interface::ObjectField {
                    name: "appServiceSlotName".into(),
                    value: &app_service_slot_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceName".into(),
                },
                register_interface::ResultField {
                    name: "appServiceSlotName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ActiveSlotResult {
            app_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceName").unwrap(),
            ),
            app_service_slot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceSlotName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
