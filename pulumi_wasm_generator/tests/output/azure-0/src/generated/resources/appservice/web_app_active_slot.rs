/// Manages a Web App Active Slot.
///
/// ## Example Usage
///
/// ### Windows Web App
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
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
///     let exampleWebAppActiveSlot = web_app_active_slot::create(
///         "exampleWebAppActiveSlot",
///         WebAppActiveSlotArgs::builder()
///             .slot_id("${exampleWindowsWebAppSlot.id}")
///             .build_struct(),
///     );
///     let exampleWindowsWebApp = windows_web_app::create(
///         "exampleWindowsWebApp",
///         WindowsWebAppArgs::builder()
///             .location("${exampleServicePlan.location}")
///             .name("example-windows-web-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleWindowsWebAppSlot = windows_web_app_slot::create(
///         "exampleWindowsWebAppSlot",
///         WindowsWebAppSlotArgs::builder()
///             .app_service_id("${exampleWindowsWebApp.name}")
///             .name("example-windows-web-app-slot")
///             .site_config(WindowsWebAppSlotSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Linux Web App
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       name: example-plan
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       osType: Linux
///       skuName: P1v2
///   exampleLinuxWebApp:
///     type: azure:appservice:LinuxWebApp
///     name: example
///     properties:
///       name: example-linux-web-app
///       resourceGroupName: ${example.name}
///       location: ${exampleServicePlan.location}
///       servicePlanId: ${exampleServicePlan.id}
///       siteConfig: {}
///   exampleLinuxWebAppSlot:
///     type: azure:appservice:LinuxWebAppSlot
///     name: example
///     properties:
///       name: example-linux-web-app-slot
///       appServiceName: ${exampleLinuxWebApp.name}
///       location: ${exampleServicePlan.location}
///       servicePlanId: ${exampleServicePlan.id}
///       siteConfig: {}
///   exampleWebAppActiveSlot:
///     type: azure:appservice:WebAppActiveSlot
///     name: example
///     properties:
///       slotId: ${exampleLinuxWebAppSlot.id}
/// ```
///
/// ## Import
///
/// a Web App Active Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/webAppActiveSlot:WebAppActiveSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1"
/// ```
///
pub mod web_app_active_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAppActiveSlotArgs {
        /// The swap action should overwrite the Production slot's network configuration with the configuration from this slot. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub overwrite_network_config: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Slot to swap with `Production`.
        #[builder(into)]
        pub slot_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAppActiveSlotResult {
        /// The timestamp of the last successful swap with `Production`.
        pub last_successful_swap: pulumi_wasm_rust::Output<String>,
        /// The swap action should overwrite the Production slot's network configuration with the configuration from this slot. Defaults to `true`. Changing this forces a new resource to be created.
        pub overwrite_network_config: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Slot to swap with `Production`.
        pub slot_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebAppActiveSlotArgs,
    ) -> WebAppActiveSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let overwrite_network_config_binding = args
            .overwrite_network_config
            .get_output(context)
            .get_inner();
        let slot_id_binding = args.slot_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/webAppActiveSlot:WebAppActiveSlot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "overwriteNetworkConfig".into(),
                    value: &overwrite_network_config_binding,
                },
                register_interface::ObjectField {
                    name: "slotId".into(),
                    value: &slot_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebAppActiveSlotResult {
            last_successful_swap: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastSuccessfulSwap"),
            ),
            overwrite_network_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("overwriteNetworkConfig"),
            ),
            slot_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("slotId")),
        }
    }
}
